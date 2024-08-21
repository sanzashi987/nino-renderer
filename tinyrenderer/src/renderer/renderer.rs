use super::{camera::Camera, shader};
use crate::{
  data_array::{ColorBuffer, DepthBuffer},
  math::{Barycentric, BoundaryBox, Mat4, Vec2, Vec4},
  model::Scene,
  obj_loader::{
    material::{self, Material, MtlStores, Texture},
    shader::{GLTypes, GlTypeMap, Shader, Uniform, Varyings},
  },
};
/// It means that the bi-unit cube [-1,1]*[-1,1]*[-1,1]
/// is mapped onto the screen cube [x,x+w]*[y,y+h]*[0,d].
/// Right, cube, and not a rectangle,
/// this is because of the depth computations with the z-buffer.
///  Here d is the resolution of the z-buffer.
/// I like to have it equal to 255 because of simplicity of
/// dumping black-and-white images of the z-buffer for debugging.
pub struct Viewport {
  x: f32,
  y: f32,
  w: f32,
  h: f32,
  d: f32,
  viewport_matrix: Mat4,
}

impl Viewport {
  pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
    let mut viewport = Self {
      x,
      y,
      w,
      h,
      d: 1.0,
      viewport_matrix: Mat4::identity(),
    };

    viewport.recompute_matrix();
    // println!("{:?}", viewport.viewport_matrix);
    viewport
  }

  #[rustfmt::skip]
  pub fn recompute_matrix(&mut self) {
    let half_w = self.w/2.0;
    let half_h = self.h/2.0;
    let half_d = self.d/2.0;

    self.viewport_matrix = Mat4::from_row(&[
     half_w , 0.0     , 0.0   , self.x + half_w,
     0.0    , -half_h , 0.0   , self.y + half_h,
     0.0    , 0.0     , half_d, half_d,
     0.0    , 0.0     , 0.0   , 1.0
    ]);
  }

  pub fn get_viewport_matrix(&self) -> &Mat4 {
    &self.viewport_matrix
  }
}

macro_rules! f {
  ($tt:tt) => {
    format!($tt)
  };
}

enum ShadowMapType {
  BasicShadowMap,
  PCFShadowMap,
  PCFSoftShadowMap,
  VSMShadowMap,
}

struct ShadowMap {
  enabled: bool,
  map_type: ShadowMapType,
}

pub struct Renderer {
  viewport: Viewport,
  pub camera: Camera,
  color: ColorBuffer,
  depth: DepthBuffer,
  stores: MtlStores,
  default_shader: Shader,
  blend: bool,
  shadow_map: bool,
  cull:bool
}

impl Renderer {
  pub fn new(w: u32, h: u32) -> Self {
    let mut depth = DepthBuffer::new(w, h);
    depth.clear(std::f32::MAX);

    Self {
      viewport: Viewport::new(0.0, 0.0, w as f32, h as f32),
      camera: Camera::new(w as f32, h as f32),
      color: ColorBuffer::new(w, h),
      depth,
      stores: Default::default(),
      default_shader: Default::default(),
      blend: false,
      shadow_map: false,
    }
  }

  pub fn render(&mut self, scene: &Scene, model_matrix: Mat4, material: &Material) {
    let width = self.color.width();
    let height = self.color.height();

    let frustum: &super::camera::Frustum = self.camera.get_frustum();
    let viewport_matrix = self.viewport.get_viewport_matrix();

    let view_matrix = *(self.camera.get_view_matarix());
    let projection_matrix = *(frustum.get_projection_matrix());
    let mvp_it = (view_matrix * model_matrix).inverse_transpose();

    let global_uniforms: GlTypeMap = GlTypeMap::from([
      (f!("model_matrix"), GLTypes::Mat4(model_matrix)),
      (f!("view_matrix"), GLTypes::Mat4(view_matrix)),
      (f!("projection_matrix"), GLTypes::Mat4(projection_matrix)),
      (f!("viewport_matrix"), GLTypes::Mat4(*viewport_matrix)),
      (f!("mv_it"), GLTypes::Mat4(mvp_it.unwrap_or_default())),
    ]);

    // todo make material mutable then it can call the mutable shaders
    for model in &scene.models {
      let vertices = &model.vertices;
      let mut uniforms = Uniform::new(&global_uniforms, Default::default());
      // let material = model
      //   .get_material()
      //   .map_or(None, |id| scene.stores.materials.get_material_by_id(id));
      let material = Some(material);
      let shader = material.map(|m| &m.shader).unwrap_or(&self.default_shader);
      for i in 0..vertices.len() / 3_usize {
        let index = (i * 3) as usize;
        let mut vertices = [vertices[index], vertices[index + 1], vertices[index + 2]];
        let mut varyings = Varyings::default();

        let mut index = 0.0;
        for v in &mut vertices {
          uniforms.set("vertex_index", GLTypes::Float(index as f32));
          *v = shader.run_vertex(v, &uniforms, &mut varyings);
          index += 1.0;
        }

        // restore the x,y,z  with 1/w, as the computation times `w` before

        // store the rhw and perform the v.position.w
        for v in &mut vertices {
          v.rhw = 1.0 / v.position.w;
          v.position /= v.position.w;
        }

        for v in &mut vertices {
          v.position = *viewport_matrix * v.position;
        }

        let vertices_2d = vertices.map(|v| v.position.truncate_to_vec2());

        let BoundaryBox {
          x_max,
          x_min,
          y_max,
          y_min,
        } = BoundaryBox::new(&vertices_2d, width as f32, height as f32);

        for x in (x_min as u32)..(x_max as u32 + 1) {
          for y in (y_min as u32)..(y_max as u32 + 1) {
            let barycentric = Barycentric::new(&Vec2::new(x as f32, y as f32), &vertices_2d);

            if !barycentric.is_inside() {
              continue;
            }

            let depth = barycentric.apply_weight(&vertices.map(|v| v.position.z));

            if self.depth.get(x, y) >= depth {
              self.depth.set(x, y, depth);

              let color = shader.run_fragment(
                &vertices,
                &barycentric,
                &uniforms,
                &varyings,
                &self.stores.texutres,
              );

              // let material = model.get_material().unwrap();
              // let diffuse_texture = material.texture_map.diffuse.unwrap();

              // let color = texture.get_pixel(vt);

              self.color.set(x, y, &color);
            }
          }
        }
      }
    }
  }

  pub fn load_texture(&mut self, filepath: &str, name: &str) {
    let _ = self.stores.texutres.load(filepath, name);
  }

  pub fn take_color(&mut self) -> ColorBuffer {
    let w = self.color.width();
    let h = self.color.height();
    self.depth.clear(std::f32::MAX);

    std::mem::replace(&mut self.color, ColorBuffer::new(w, h))
  }
}

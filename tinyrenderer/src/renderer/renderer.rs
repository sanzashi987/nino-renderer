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
      d: 1.0, //255.0,
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

pub struct Renderer {
  viewport: Viewport,
  pub camera: Camera,
  color: ColorBuffer,
  depth: DepthBuffer,
  stores: MtlStores,
  default_shader: Shader,
}

impl Renderer {
  pub fn new(w: u32, h: u32) -> Self {
    let mut depth = DepthBuffer::new(w, h);
    depth.clear(std::f32::MIN);

    Self {
      viewport: Viewport::new(0.0, 0.0, w as f32, h as f32),
      camera: Camera::new(w as f32, h as f32),
      color: ColorBuffer::new(w, h),
      depth,
      stores: Default::default(),
      default_shader: Default::default(),
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
      (format!("model_matrix"), GLTypes::Mat4(model_matrix)),
      (format!("view_matrix"), GLTypes::Mat4(view_matrix)),
      (
        format!("projection_matrix"),
        GLTypes::Mat4(projection_matrix),
      ),
      (format!("mv_it"), GLTypes::Mat4(mvp_it.unwrap_or_default())),
    ]);

    // let mvp = projection_matrix * view_matrix * model_matrix;
    // dbg!(mvp);
    // dbg!(view_matrix * model_matrix);
    // dbg!((view_matrix * model_matrix).inverse_transpose());
    // dbg!(mvp_it.unwrap_or_default().transpose() * mvp);

    for model in &scene.models {
      let vertices = &model.vertices;
      let uniforms = Uniform::new(&global_uniforms, Default::default());
      // let material = model
      //   .get_material()
      //   .map_or(None, |id| scene.stores.materials.get_material_by_id(id));
      let material = Some(material);
      let shader = material.map(|m| &m.shader).unwrap_or(&self.default_shader);
      for i in 0..vertices.len() / 3_usize {
        let index = (i * 3) as usize;
        let mut vertices = [vertices[index], vertices[index + 1], vertices[index + 2]];
        let mut varyings = Varyings::default();

        for v in &mut vertices {
          *v = shader.run_vertex(v, &uniforms, &mut varyings);
        }

        // restore the x,y,z  with 1/w, as the computation times `w` before

        // store the rhw and perform the v.position.w
        for v in &mut vertices {
          v.rhw = -1.0 / v.position.w;

          v.position /= v.position.w;
          // v.position.z = -v.position.w;
          // v.position.x /= v.position.w;
          // v.position.y /= v.position.w;
          // v.position.w = 1.0;
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
            let rhws = vertices.map(|v| v.rhw);
            let inv_z = barycentric.apply_weight(&vertices.map(|v| v.rhw));
            let z = 1.0 / inv_z;

            if self.depth.get(x, y) < z {
              self.depth.set(x, y, z);

              // let vt = barycentric.apply_weight(&vertices.map(|v| v.texture.unwrap() * v.rhw)) * z;

              let color = shader.run_fragment(
                &barycentric,
                &uniforms,
                &varyings,
                &self.stores.texutres,
                rhws,
                z,
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
    self.depth.clear(std::f32::MIN);

    std::mem::replace(&mut self.color, ColorBuffer::new(w, h))
  }
}

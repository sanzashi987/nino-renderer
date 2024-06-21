use renderer_macro_derive::renderer;

use crate::{
  bresenham_line,
  camera::Camera,
  cohen_sutherland,
  image::{ColorAttachment, DepthAttachment},
  math::{Mat4, Vec2, /* Vec3, */ Vec4},
  renderer::*,
  scanline::{self, near_plane_clip},
  shader::{self, attributes_foreach, Shader, Uniforms, Vertex},
  texture::{Texture, TextureStore},
};

// #[derive(RendererCommon)]
#[renderer]
pub struct Renderer;

impl RendererDraw for Renderer {
  fn draw_triangle(&mut self, model: &Mat4, vertices: &[Vertex], texture_store: &TextureStore) {
    for i in 0..vertices.len() / 3_usize {
      let index = (i * 3) as usize;
      let vertices = [vertices[index], vertices[index + 1], vertices[index + 2]];
      match self.rasterize_triangle(model, vertices, texture_store) {
        RasterizeResult::Ok | RasterizeResult::Discard => {}
        RasterizeResult::GenerateNewFace => {
          for i in 0..self.cliped_triangles.len() / 3 {
            let vs = [
              self.cliped_triangles[i * 3],
              self.cliped_triangles[i * 3 + 1],
              self.cliped_triangles[i * 3 + 2],
            ];
            match self.rasterize_triangle(model, vs, texture_store) {
              RasterizeResult::Ok => {}
              RasterizeResult::Discard | RasterizeResult::GenerateNewFace => {
                panic!("discard or generate new face for clipped face")
              }
            }
          }
          self.cliped_triangles.clear();
        }
      }
    }
  }
}
impl Renderer {
  fn rasterize_triangle(
    &mut self,
    model: &Mat4,
    mut vertices: [Vertex; 3],
    // texture: Option<&Texture>,
    texture_store: &TextureStore,
  ) -> RasterizeResult {
    let frustum = self.camera.get_frustum();

    for v in &mut vertices {
      // v.position = *model * v.position;
      *v = self
        .shader
        .call_vertex_shading(v, &self.uniforms, texture_store);
    }

    // M transform
    for v in &mut vertices {
      v.position = *model * v.position;
    }

    // cull clip
    if should_cull(
      &vertices.map(|v| v.position.truncated_to_vec3()),
      self.camera.get_view_direction(),
      self.front_face,
      self.cull,
    ) {
      return RasterizeResult::Discard;
    }

    // View Transform
    for v in &mut vertices {
      v.position = *self.camera.get_view_matarix() * v.position;
    }

    // Projection transform
    for v in &mut vertices {
      v.position = *frustum.get_mat() * v.position;
    }

    // frustum clip
    if vertices
      .iter()
      .all(|v| !frustum.contains(&v.truncated_to_vec3()))
    {
      return RasterizeResult::Discard;
    }

    // near plane clip
    if vertices
      .iter()
      .any(|v| v.position.z > self.camera.get_frustum().near())
    {
      let (face1, face2) = near_plane_clip(&vertices, self.camera.get_frustum().near());

      self.cliped_triangles.extend(face1.iter());
      if let Some(face) = face2 {
        self.cliped_triangles.extend(face.iter());
      }
      return RasterizeResult::GenerateNewFace;
    }

    // restore z from w (original z in 3D)
    for v in &mut vertices {
      v.position.z = -v.position.w * frustum.near();
    }

    // restore y/x from w (projected x,y in 2D)
    for v in &mut vertices {
      v.position.x /= v.position.w;
      v.position.y /= v.position.w;
      v.position.w = 1.0;
    }

    // viewport affine, from [-1, 1]^2 to [0, W - 1] x [0, H - 1]
    for v in &mut vertices {
      v.position.x =
        (v.position.x + 1.0) * 0.5 * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32;
      v.position.y =
        (v.position.y + 1.0) * 0.5 * (self.viewport.h as f32 - 1.0) + self.viewport.y as f32;
    }

    if self.wireframe_mode {
      for i in 0..3 {
        let mut v1 = vertices[i];
        let mut v2 = vertices[(i + 1) % 3];
      }
    } else {
      // for each triangle , cut in two possible trapezoid
      let [trap1, trap2] = &mut scanline::Trapezoid::from_triangle(&vertices);
      // rasterization
      if let Some(trap) = trap1 {
        self.draw_trapezoid(trap, texture_store);
      }
      if let Some(trap) = trap2 {
        self.draw_trapezoid(trap, texture_store);
      }
    }
    RasterizeResult::Ok

    // let length = vertices.len();
    // for i in 0..length {
    //   let p1 = &vertices[i];
    //   let p2 = &vertices[(i + 1) % length];

    //   self.draw_line(&p1, &p2, texture);
    // }
  }
}
impl Renderer {
  fn draw_line(&mut self, p0: &Vec2, p1: &Vec2, color: &Vec4) {
    // let rect_min = Vec2 { x: 50.0, y: 50.0 };
    let rect_min = Vec2::zero();
    let rect_max = Vec2::new(
      self.color.width() as f32 - 1.0,
      self.color.height() as f32 - 1.0,
    );

    let res = cohen_sutherland::clip(&p0, &p1, &rect_min, &rect_max);

    // if let Some((next_p0, next_p1)) = res {}

    match res {
      Some((next_p0, next_p1)) => bresenham_line::draw_line(
        next_p0.x as i32,
        next_p0.y as i32,
        next_p1.x as i32,
        next_p1.y as i32,
        color,
        &mut self.color,
      ),
      None => {}
    }
  }

  pub fn draw_trapezoid(&mut self, trap: &mut scanline::Trapezoid, texture_store: &TextureStore) {
    let top = trap.top.ceil().max(0.0) as i32;
    let bottom = trap.bottom.ceil().min(self.color.height() as f32 - 1.0) as i32 - 1;
    let mut y = top as f32;

    // reciprocal vertex attributes, (all attribute divided by the original z)
    shader::vertex_rhw_init(&mut trap.left.v1);
    shader::vertex_rhw_init(&mut trap.left.v2);
    shader::vertex_rhw_init(&mut trap.right.v1);
    shader::vertex_rhw_init(&mut trap.right.v2);

    while y <= bottom as f32 {
      let scanline = scanline::Scanline::from_trapezoid(&trap, y);
      self.draw_scanline(&scanline, texture_store);
      y += 1.0;
    }
  }

  pub fn draw_scanline(&mut self, scanline: &scanline::Scanline, texture_store: &TextureStore) {
    let mut vertex = scanline.vertex;
    let y: u32 = scanline.y as u32;
    let mut width = scanline.width;
    while width > 0.0 {
      let x = vertex.position.x;
      if x >= 0.0 && x < self.color.width() as f32 {
        let real_z = 1.0 / vertex.position.z;
        if self.depth.get(x as u32, y) <= real_z {
          // local copy
          let mut attr_local = vertex.attributes;

          // perspective correction restore with `z`(precompute and store in `z` inside the `rhw_init`)
          attributes_foreach(&mut attr_local, |v| v / vertex.position.z);

          // let textcoord = attr_local.vec2[ATTR_TEXCOORD];
          // let color = attr_local.vec4[ATTR_COLOR]
          //   * match texture {
          //     Some(texture) => renderer::texture_sample(&texture, &textcoord),
          //     None => Vec4::new(1.0, 1.0, 1.0, 1.0),
          //   };
          let color = self
            .shader
            .call_fragment_shading(&attr_local, &self.uniforms, texture_store);
          self.color.set(x as u32, y, &color);
          self.depth.set(x as u32, y, real_z);
        }
      }

      width -= 1.0;
      vertex.position += scanline.step.position;
      // apply the interpolation
      vertex.attributes = shader::interp_attributes(
        &vertex.attributes,
        &scanline.step.attributes,
        |v1, v2, _| v1 + v2,
        0.0,
      )
    }
  }
}

use crate::{
  camera::Camera,
  image::ColorAttachment,
  renderer::{RendererInterface, Viewport},
};

pub struct Renderer {
  color: ColorAttachment,
  camera: Camera,
  viewport: Viewport,
}

impl RendererInterface for Renderer {
  fn clear(&mut self, color: &crate::math::Vec4) {
    self.color.clear(color);
  }

  fn get_canvas_width(&self) -> u32 {
    self.color.width()
  }

  fn get_canvas_height(&self) -> u32 {
    self.color.height()
  }

  fn get_frame_image(&self) -> &[u8] {
    self.color.data()
  }

  fn draw_triangle(
    &mut self,
    model: &crate::math::Mat4,
    vertices: &[crate::vertex::Vertex],
    count: u32,
    texture: Option<&crate::texture::Texture>,
  ) {
    for i in 0..count {
      let index = (i * 3) as usize;
      let mut vertices = [vertices[index], vertices[index + 1], vertices[index + 2]];

      for v in &mut vertices {
        v.position = *model * v.position;
      }

      for v in &mut vertices {
        v.position = *self.camera.get_frustum().get_mat() * v.position;
      }

      for v in &mut vertices {
        v.position.z = -v.position.w;
      }

      for v in &mut vertices {
        v.position.x /= v.position.w;
        v.position.y /= v.position.w;
      }

      for v in &mut vertices {
        v.position.x =
          (v.position.x + 1.0) * 0.5 * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32;
        v.position.y =
          (v.position.y + 1.0) * 0.5 * (self.viewport.h as f32 - 1.0) + self.viewport.y as f32;
      }
    }
  }
}

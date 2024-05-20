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
    todo!()
  }
}

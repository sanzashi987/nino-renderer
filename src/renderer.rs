use crate::{
  math::{Mat4, Vec4},
  vertex::Vertex,
};
pub struct Viewport {
  pub x: i32,
  pub y: i32,
  pub w: u32,
  pub h: u32,
}

pub trait RendererInterface {
  fn clear(&mut self, color: &Vec4);
  fn get_canvas_width(&self) -> u32;
  fn get_canvas_height(&self) -> u32;
  fn get_frame_image(&self) -> &[u8];
  fn draw_triangle(
    &mut self,
    model: &Mat4,
    vertices: &[Vertex],
    count: u32,
    texture: Option<&image::DynamicImage>,
  );
}

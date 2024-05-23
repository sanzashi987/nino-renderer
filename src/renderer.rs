use crate::{
  math::{Mat4, Vec2, Vec4},
  texture::Texture,
  shader::Vertex,
};

pub const ATTR_COLOR: usize = 0;
pub const ATTR_TEXCOORD: usize = 1;
pub const UNIFORM_TEXTURE: u32 = 0;

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
    texture: Option<&Texture>,
  );
}

pub fn texture_sample(texture: &Texture, textcoord: &Vec2) -> Vec4 {
  let x = (textcoord.x * (texture.width() - 1) as f32) as u32;
  let y = (textcoord.y * (texture.height() - 1) as f32) as u32;

  texture.get_pixel(x, y)
}

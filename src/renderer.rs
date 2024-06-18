use crate::{
  math::{Mat4, Vec2, Vec3, Vec4},
  shader::{Shader, Uniforms, Vertex},
  texture::{Texture, TextureStore},
};

pub struct Viewport {
  pub x: i32,
  pub y: i32,
  pub w: u32,
  pub h: u32,
}

pub trait RendererDerive {
  fn clear(&mut self, color: &Vec4);
  fn clear_depth(&mut self);
  fn get_canvas_width(&self) -> u32;
  fn get_canvas_height(&self) -> u32;
  fn get_frame_image(&self) -> &[u8];
  fn get_shader(&mut self) -> &mut Shader;
  fn get_uniforms(&mut self) -> &mut Uniforms;
  fn enable_framework(&mut self);
  fn disable_framework(&mut self);
  fn toggle_framework(&mut self);
}

pub trait RendererDraw {
  fn draw_triangle(
    &mut self,
    model: &Mat4,
    vertices: &[Vertex],
    // count: u32,
    // texture: Option<&Texture>,
    texture_store: &TextureStore,
  );
}

pub trait RendererInterface: RendererDerive + RendererDraw {}

pub fn texture_sample(texture: &Texture, textcoord: &Vec2) -> Vec4 {
  let x = (textcoord.x * (texture.width() - 1) as f32) as u32;
  let y = (textcoord.y * (texture.height() - 1) as f32) as u32;

  texture.get_pixel(x, y)
}

#[derive(Clone, Copy, Debug)]
pub enum FaceCull {
  Front,
  Back,
  None,
}

#[derive(Clone, Copy, Debug)]
pub enum FrontFace {
  /**
   * clockwise
   */
  CW,
  /**
   * counter clockwise
   */
  CCW,
}

pub(crate) fn should_cull(
  positions: &[Vec3; 3],
  view_direction: &Vec3,
  face: FrontFace,
  cull: FaceCull,
) -> bool {
  let norm = (positions[1] - positions[0]).cross(&(positions[2] - positions[1]));
  let is_front_face = match face {
    FrontFace::CW => norm.dot(&view_direction) > 0.0,
    FrontFace::CCW => norm.dot(&view_direction) <= 0.0,
  };

  match cull {
    FaceCull::Front => is_front_face,
    FaceCull::Back => !is_front_face,
    FaceCull::None => false,
  }
}

pub enum RasterizeResult {
  Ok,
  Discard,
  GenerateNewFace,
}

use std::collections::btree_set::Iter;

use crate::{
  bresenham_line::Bresenham,
  image::{ColorAttachment, DepthAttachment},
  line::Line,
  math::{Mat4, Vec2, Vec3, Vec4},
  shader::{
    attributes_foreach, interp_attributes, vertex_rhw_init, FragmentShading, Shader, Uniforms,
    Vertex,
  },
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
  fn enable_wireframe(&mut self);
  fn disable_wireframe(&mut self);
  fn toggle_wireframe(&mut self);
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

pub(crate) fn rasterize_wireframe(
  vertices: &[Vertex; 3],
  // line: Line,
  fragment_shader: &FragmentShading,
  uniforms: &Uniforms,
  texture_store: &TextureStore,
  color: &mut ColorAttachment,
  depth: &mut DepthAttachment,
) {
  // 0-1, 1-2, 2-0
  for i in 0..3 {
    let mut v1 = vertices[i];
    let mut v2 = vertices[(i + 1) % 3];

    vertex_rhw_init(&mut v1);
    vertex_rhw_init(&mut v2);
    let line = Line::new(v1, v2);
    let mut bresenham = Bresenham::new(
      line.start.position.truncated_to_vec2(),
      line.end.position.truncated_to_vec2(),
      Vec2::zero(),
      Vec2::new(color.width() as f32 - 1.0, color.height() as f32 - 1.0),
    );
    if let Some(instance) = &mut bresenham {
      let mut position = instance.next();
      let mut vertex = line.start;
      while position.is_some() {
        let (x, y) = position.unwrap();
        let z = 1.0 / vertex.position.z;

        let x = x as u32;
        let y = y as u32;
        if depth.get(x, y) <= z {
          let mut attr = vertex.attributes;
          attributes_foreach(&mut attr, |a| a * z);

          let pixel = fragment_shader(&attr, &uniforms, &texture_store);
          color.set(x, y, &pixel);
          depth.set(x, y, z);
        }

        vertex.position += line.step.position;
        vertex.attributes = interp_attributes(
          &vertex.attributes,
          &line.step.attributes,
          |v1, v2, _| return v1 + v2,
          1.0,
        );
        position = instance.next();
      }
    }
  }
}

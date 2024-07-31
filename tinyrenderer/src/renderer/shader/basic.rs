use std::collections::HashMap;

use crate::{
  math::{Mat4, Vec2, Vec3, Vec4},
  model::Vertex,
};
#[derive(Debug, Clone, Copy)]
pub enum GLTypes {
  Int(i32),
  Float(f32),
  Vec2(Vec2),
  Vec3(Vec3),
  Vec4(Vec4),
}

pub struct GLMVP<'a> {
  model_matrix: &'a Mat4,
  view_matrix: &'a Mat4,
  projection_matrix: &'a Mat4,
  viewport_matrix: &'a Mat4,
}

pub struct Varying {
  data: HashMap<String, GLTypes>,
}

impl Varying {
  pub fn new() -> Self {
    Self {
      data: HashMap::new(),
    }
  }
  pub fn set(&mut self, key: &str, val: GLTypes) {
    self.data.insert(key.to_string(), val);
  }

  pub fn get(&self, key: &str) -> Option<GLTypes> {
    self.data.get(key).map(|e| *e)
  }
}

pub type Uniform = Varying;

pub trait Shader {
  fn vertex(v: &Vertex, u: &Uniform, va: &mut Varying) -> Vertex;
  fn fragment(v: &Vertex, u: &Uniform, va: &Varying) -> Vec4;
}

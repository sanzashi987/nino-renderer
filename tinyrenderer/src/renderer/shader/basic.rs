use std::{
  collections::HashMap,
  ops::{Add, Mul},
};

use crate::{
  math::{Vec2, Vec3, Vec4},
  model::Vertex,
};
pub enum GLTypes {
  Int(i32),
  Float(f32),
  Vec2(Vec2),
  Vec3(Vec3),
  Vec4(Vec4),
}

struct Varying {
  data: HashMap<String, GLTypes>,
}

type Uniform = Varying;

struct Shader {
  vertex: Box<dyn Fn(&Vertex, &Uniform, &Varying) -> Vertex>,
  fragment: Box<dyn Fn() -> Vec4>,
}

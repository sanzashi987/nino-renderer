use super::basic::{Shader, Uniform, Varying};
use crate::{math::Vec4, model::Vertex};

pub struct GouraudShader {}

impl Shader for GouraudShader {
  fn vertex(v: &Vertex, u: &Uniform, va: &mut Varying) -> Vertex {}

  fn fragment(v: &Vertex, u: &Uniform, va: &Varying) -> Vec4 {}
}

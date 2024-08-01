use super::basic::{Shader, Uniform, Varying};
use crate::{
  math::{Vec3, Vec4},
  model::Vertex,
};

pub struct GouraudShader {
  pub light_dir: Vec3,
}

impl Shader for GouraudShader {
  fn vertex(&self, v: &Vertex, u: &Uniform, va: &mut Varying) -> Vertex {
    if let Some(normal) = v.normal {
      va.set(
        "light-intense",
        super::basic::GLTypes::Float(normal.dot(&self.light_dir)),
      )
    }

    *v
  }

  fn fragment(&self, v: &Vertex, u: &Uniform, va: &Varying) -> Vec4 {
    todo!()
  }
}

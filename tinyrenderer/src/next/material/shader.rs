use super::super::core::buffer_geometry::Attribute;

use super::super::core::unifrom::Uniform;
use super::super::core::varying::Varying;
use std::fmt::Debug;

type VertexShader = Box<dyn Fn(&Attribute, &Uniform, &mut Varying)>;
type FragmentShader = Box<dyn Fn(&Uniform, &Varying) -> bool>;

pub struct Shader {
  pub vertex: VertexShader,
  pub fragment: FragmentShader,
}

impl Debug for Shader {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Shader")
      .field("vertex", &"/** vertex clousure */".to_string())
      .field("fragment", &"/** fragment clousure */".to_string())
      .finish()
  }
}

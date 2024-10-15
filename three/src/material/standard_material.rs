use crate::math::Vec3;

use super::{
  material::{define_uniform_attr, BasicMaterial, ConvertUniform},
  shader::DefineShader,
};
#[derive(Debug, Default)]
pub struct StandardMeshAttribute {
  pub ambient: Option<Vec3>,
  pub diffuse: Option<Vec3>,
  pub specular: Option<Vec3>,
  pub emissive_coeficient: Option<Vec3>,
  pub specular_exponent: Option<f32>,
  pub dissolve: Option<f32>,
  pub transmission_filter: Option<Vec3>,
  pub optical_density: Option<f32>,
  pub illum: Option<u32>,
}

impl ConvertUniform for StandardMeshAttribute {
  fn to_uniform(&self) -> crate::core::unifrom::Uniform {
    let mut uniform = Default::default();
    define_uniform_attr!(uniform;
      ambient,
      diffuse,
      specular,
      emissive_coeficient,
      specular_exponent,
      dissolve,
      transmission_filter,
      optical_density,
      illum,
    );
    uniform
  }
}

pub struct StandardShader {}
impl DefineShader for StandardShader {
  fn vertex() -> super::shader::VertexShader {
    todo!()
  }

  fn fragment() -> super::shader::FragmentShader {
    todo!()
  }
}

pub type StandardMeshMaterial = BasicMaterial<StandardMeshAttribute, StandardShader>;

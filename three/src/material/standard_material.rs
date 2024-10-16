use crate::{
  core::{buffer_geometry::Attribute, uniform::Uniform, varying::Varying},
  math::Vec3,
};

use super::{
  material::{define_uniform_attr, BasicMaterial, ConvertUniform},
  shader::{DefineShader, GlPerFragment, GlPerVertex},
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
  fn to_uniform(&self) -> crate::core::uniform::Uniform {
    let mut uniform: crate::core::uniform::Uniform = Default::default();
    define_uniform_attr!(uniform;self;
      ambient,
      diffuse,
      specular,
      emissive_coeficient,
      specular_exponent,
      dissolve,
      transmission_filter,
      optical_density,
      illum
    );
    uniform
  }
}

fn standard_vertex_shader(
  attribute: &Attribute,
  uniform: &Uniform,
  varying: &mut Varying,
  gl_vertex: &mut GlPerVertex,
) {
}

fn standard_fragment_shader(
  uniform: &Uniform,
  varying: &Varying,
  gl_fragment: &mut GlPerFragment,
) -> bool {
  false
}

pub struct StandardShader {}
impl DefineShader for StandardShader {
  fn vertex() -> super::shader::VertexShader {
    Box::new(standard_vertex_shader)
  }

  fn fragment() -> super::shader::FragmentShader {
    Box::new(standard_fragment_shader)
  }
}

pub type StandardMeshMaterial = BasicMaterial<StandardMeshAttribute, StandardShader>;

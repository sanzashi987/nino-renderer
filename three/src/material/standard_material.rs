use crate::{
  core::{buffer_geometry::Attribute, uniform::Uniform, varying::Varying},
  loaders::mtl_loader::MtlData,
  math::Vec3,
};

use super::{
  material::{define_material_attribute, BasicMaterial},
  shader::{DefineShader, GlPerFragment, GlPerVertex},
};

define_material_attribute!(
  StandardMeshAttribute;
  Ns->specular_exponent: f32,
  Ka->ambient:Vec3,
  Kd->diffuse:Vec3,
  Ks->specular:Vec3,
  Ke->emissive_coeficient:Vec3,
  Tf->transmission_filter:Vec3,
  Ni->optical_density:f32,
  d->dissolve:f32,
  // Tr->dissolve:f32,
  illum->illum:u32
);

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

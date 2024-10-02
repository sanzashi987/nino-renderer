use crate::core::unifrom::Uniform;

use super::{
  material::{BasicMaterial, ConvertUniform},
  shader::DefineShader,
};
pub enum DepthPacking {
  BasicDepthPacking = 3200,
  RGBADepthPacking = 3201,
  RGBDepthPacking = 3202,
  RGDepthPacking = 3203,
}

impl Default for DepthPacking {
  fn default() -> Self {
    Self::BasicDepthPacking
  }
}

struct MeshDepthAttribute {
  depth_packing: DepthPacking,
  wireframe: bool,
  wirefame_linewidth: u8,
}

impl ConvertUniform for MeshDepthAttribute {
  fn to_uniform(&self) -> Uniform {
    let res = Uniform::default();

    res
  }
}

struct DepthShader {}

impl DefineShader for DepthShader {
  fn vertex() -> super::shader::VertexShader {
    todo!()
  }

  fn fragment() -> super::shader::FragmentShader {
    todo!()
  }
}

pub type MeshDepthMaterial = BasicMaterial<MeshDepthAttribute, DepthShader>;

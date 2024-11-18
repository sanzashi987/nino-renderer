use crate::core::uniform::{Uniform, UniformTypeEnum};

use super::{
  material::{BasicMaterial, ToUniform},
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
}

impl ToUniform for MeshDepthAttribute {
  fn to_uniform(&self) -> Uniform {
    let mut res = Uniform::default();
    res.insert("wireframe", UniformTypeEnum::Bool(self.wireframe));
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

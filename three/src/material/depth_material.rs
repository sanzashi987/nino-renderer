use crate::core::uniform::{Uniform, UniformTypeEnum};

use super::{
  material::{BasicMaterial, ToUniform},
  shader::DefineShader,
};

#[derive(Debug, Clone, Copy)]
pub enum DepthPacking {
  BasicDepthPacking = 3200,
  RGBADepthPacking = 3201,
  RGBDepthPacking = 3202,
  RGDepthPacking = 3203,
}

impl Into<f32> for DepthPacking {
  fn into(self) -> f32 {
    match self {
      DepthPacking::BasicDepthPacking => 3200f32,
      DepthPacking::RGBADepthPacking => 3201f32,
      DepthPacking::RGBDepthPacking => 3202f32,
      DepthPacking::RGDepthPacking => 3203f32,
    }
  }
}

impl Default for DepthPacking {
  fn default() -> Self {
    Self::BasicDepthPacking
  }
}

#[derive(Debug, Default)]
pub struct MeshDepthAttribute {
  depth_packing: DepthPacking,
}

impl ToUniform for MeshDepthAttribute {
  fn to_uniform(&self) -> Uniform {
    let mut res = Uniform::default();
    let a: f32 = self.depth_packing.into();
    res.insert("wireframe", UniformTypeEnum::Float(a));
    res
  }
}

pub struct DepthShader {}

impl DefineShader for DepthShader {
  fn vertex() -> super::shader::VertexShader {
    todo!()
  }

  fn fragment() -> super::shader::FragmentShader {
    todo!()
  }
}
pub type MeshDepthMaterial = BasicMaterial<MeshDepthAttribute, DepthShader>;

use std::collections::HashMap;

use crate::math::{Vec2, Vec3};

use super::super::objects::mesh::Mesh;

#[derive(Debug, Default, Clone, Copy)]
pub struct VertexIndex {
  pub position_index: u32,
  pub normal_index: Option<u32>,
  pub uv_index: Option<u32>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Face {
  pub vertices: [VertexIndex; 3],
}
#[derive(Debug, Default)]
pub struct TempMesh {
  pub name: String,
  pub faces: Vec<Face>,
  pub material: Option<String>,
}

pub struct TempObjInfos {
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub uvs: Vec<Vec2>,
}

pub struct ObjLoader {
  next_model_id: u32,
  loaded_objs: HashMap<u32, Mesh>,
  path_id_map: HashMap<String, u32>,
}

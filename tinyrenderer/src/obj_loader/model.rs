use crate::math::{Vec2, Vec3};

pub struct VertexPointer {
  vertex_index: u32,
  normal_index: Option<u32>,
  texture_index: Option<u32>,
}

pub struct Face {
  vertices: [VertexPointer; 3],
}

pub struct Model {
  name: String,
  faces: Vec<Face>,
  vertices: Vec<Vec3>,
  normals: Vec<Vec3>,
  texture_coordinates: Vec<Vec2>,
}

impl Model {
  // pub fn get_name(&self) -> &String {
  //   &self.name
  // }
}

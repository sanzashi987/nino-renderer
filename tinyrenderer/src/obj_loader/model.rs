use std::path::Path;

use crate::math::{Vec2, Vec3};
#[derive(Debug, Default, Clone, Copy)]
pub struct VertexPointer {
  vertex_index: u32,
  normal_index: Option<u32>,
  texture_index: Option<u32>,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Face {
  vertices: [VertexPointer; 3],
}
#[derive(Debug)]
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
  pub fn single_mode(filename: &str) -> Self {
    Self {
      name: filename.to_string(),
      faces: Default::default(),
      vertices: Default::default(),
      normals: Default::default(),
      texture_coordinates: Default::default(),
    }
  }

  pub fn add_vertex(&mut self, vertex: Vec3) {
    self.vertices.push(vertex)
  }

  pub fn add_normal(&mut self, normal: Vec3) {
    self.normals.push(normal)
  }

  pub fn add_texture_coordinate(&mut self, texture_coordinate: Vec2) {
    self.texture_coordinates.push(texture_coordinate)
  }
}

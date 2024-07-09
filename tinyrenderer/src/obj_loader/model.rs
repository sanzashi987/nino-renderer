use std::path::Path;

use crate::math::{Vec2, Vec3};

use super::ParserError;
#[derive(Debug, Default, Clone, Copy)]
pub struct VertexPointer {
  pub vertex_index: u32,
  pub normal_index: Option<u32>,
  pub texture_index: Option<u32>,
}
impl VertexPointer {
  pub fn new(vertex_index: u32, normal_index: Option<u32>, texture_index: Option<u32>) -> Self {
    Self {
      vertex_index,
      normal_index,
      texture_index,
    }
  }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Face {
  pub vertices: [VertexPointer; 3],
}
#[derive(Debug)]
pub struct Model {
  pub name: String,
  pub faces: Vec<Face>,
}

impl Model {
  // pub fn get_name(&self) -> &String {
  //   &self.name
  // }
  pub fn new(name: String) -> Self {
    Self {
      name,
      faces: Default::default(),
    }
  }

  pub fn get_faces(&self) -> &Vec<Face> {
    &self.faces
  }
}

#[derive(Debug)]
pub struct Scene {
  pub models: Vec<Model>,
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub texture_coordinates: Vec<Vec2>,
}

impl Scene {
  pub fn new() -> Self {
    Self {
      models: Vec::new(),
      vertices: Default::default(),
      normals: Default::default(),
      texture_coordinates: Default::default(),
    }
  }

  pub fn add_model(&mut self, name: String) {
    self.models.push(Model::new(name));
  }

  pub fn add_face(&mut self, face: Face) -> Result<(), ParserError> {
    self
      .models
      .last_mut()
      .ok_or(ParserError::ModelNotInit)?
      .faces
      .push(face);
    Ok(())
  }

  pub fn add_vertex(&mut self, vertex: Vec3) {
    let mut nextVertex =  vertex;
    // nextVertex.y = 1.0 - nextVertex.y;
    
    self.vertices.push(nextVertex)
  }

  pub fn add_normal(&mut self, normal: Vec3) {
    self.normals.push(normal)
  }

  pub fn add_texture_coordinate(&mut self, texture_coordinate: Vec2) {
    self.texture_coordinates.push(texture_coordinate)
  }
}

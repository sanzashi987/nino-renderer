use crate::{
  math::{Vec2, Vec3},
  utils::swap_and_move,
};

use super::{
  defines::ParserError,
  material::{Materials, MoveMaterials, Textures},
};
#[derive(Debug, Default, Clone, Copy)]
pub struct VertexIndex {
  pub position_index: u32,
  pub normal_index: Option<u32>,
  pub texture_index: Option<u32>,
}
impl VertexIndex {
  pub fn new(position_index: u32, normal_index: Option<u32>, texture_index: Option<u32>) -> Self {
    Self {
      position_index,
      normal_index,
      texture_index,
    }
  }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Face {
  pub vertices: [VertexIndex; 3],
}
#[derive(Debug, Default)]
pub struct Model {
  pub name: String,
  pub faces: Vec<Face>,
  pub material: Option<String>,
}

impl Model {
  pub fn new(name: String) -> Self {
    Self {
      name,
      faces: Default::default(),
      material: None,
    }
  }

  pub fn get_faces(&self) -> &Vec<Face> {
    &self.faces
  }
}
#[derive(Debug, Default)]
pub struct Scene {
  pub models: Vec<Model>,
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub texture_coordinates: Vec<Vec2>,
  pub materials: Materials,
}

impl Scene {
  pub fn new() -> Self {
    Self {
      models: Default::default(),
      vertices: Default::default(),
      normals: Default::default(),
      texture_coordinates: Default::default(),
      materials: Default::default(),
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
    let mut next_vertex = vertex;
    // next_vertex.y = -1.0 * next_vertex.y;
    // next_vertex.z =  -next_vertex.z;

    self.vertices.push(next_vertex)
  }

  pub fn add_normal(&mut self, normal: Vec3) {
    self.normals.push(normal)
  }

  pub fn add_texture_coordinate(&mut self, texture_coordinate: Vec2) {
    let mut next_vt = texture_coordinate;
    // next_vt.y = 1.0 - next_vt.y;

    self.texture_coordinates.push(next_vt);
  }

  pub fn bind_material(&mut self, material_name: String) -> Result<(), ParserError> {
    self
      .models
      .last_mut()
      .ok_or(ParserError::ModelNotInit)?
      .material = Some(material_name);
    Ok(())
  }
}

impl MoveMaterials for Scene {
  fn move_out_materials(&mut self) -> Materials {
    swap_and_move(&mut self.materials)
  }

  fn move_in_materials(&mut self, materials: Materials) {
    self.materials = materials;
  }
}

use crate::math::{Vec2, Vec3, Vec4};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
  pub position: Vec3,
  pub normal: Option<Vec3>,
  pub texture: Option<Vec2>,
}

pub struct Model {
  pub vertices: Vec<Vertex>,
  pub name: Option<String>,
  // pub material
}

pub struct Scene {
  pub models: Vec<Model>,
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub texture_coordinates: Vec<Vec2>,
}

pub fn init_obj(filepath: &str) {}



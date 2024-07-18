use crate::{
  math::{Vec2, Vec3, Vec4},
  obj_loader::{
    defines::ParserError,
    load_obj,
    material::{Material, Texture},
  },
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
  pub position: Vec3,
  pub normal: Option<Vec3>,
  pub texture: Option<Vec2>,
}
#[derive(Debug, Default)]
pub struct Model {
  pub vertices: Vec<Vertex>,
  pub name: String,
  pub material: Option<Material>,
}
#[derive(Debug, Default)]
pub struct Scene {
  pub models: Vec<Model>,
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub texture_coordinates: Vec<Vec2>,
}

pub fn from_obj_path(relative_path: &str) -> Result<Scene, ParserError> {
  let mut parser = load_obj(relative_path)?;
  let obj_scene = parser.parse()?;

  let mut scene: Scene = Default::default();

  for obj_model in &mut obj_scene.models {
    let mut model: Model = Default::default();
    model.name = obj_model.name.clone();

    for obj_face in &obj_model.faces {}
  }

  Ok(scene)
}

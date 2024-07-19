use std::slice::SliceIndex;

use crate::{
  math::{Vec2, Vec3, Vec4},
  obj_loader::{
    defines::ParserError,
    load_obj,
    material::{self, Material as ObjMaterial, MaterialBase, Materials, Texture, TextureMap},
    Model as ObjModel, Scene as ObjScene, VertexIndex,
  },
  utils::swap_and_move,
};

type TextureRefer<'a> = TextureMap<&'a Texture>;
impl<'a> Default for TextureRefer<'a> {
  fn default() -> Self {
    Self {
      ..Default::default()
    }
  }
}

type Material<'a> = MaterialBase<TextureRefer<'a>>;

impl<'a> Material<'a> {
  pub fn from_obj_material(obj_material: &ObjMaterial, scene: &Scene) -> Self {
    let texture_map = TextureRefer::default();
    let name = obj_material.name.clone();

    Self::from_another_material_type(obj_material, name, texture_map)
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
  pub position: Vec3,
  pub normal: Option<Vec3>,
  pub texture: Option<Vec2>,
}

impl Vertex {
  pub fn from_vertex_index(v: VertexIndex, scene: &Scene) -> Self {
    let VertexIndex {
      position_index,
      normal_index,
      texture_index,
    } = v;

    let normal = normal_index.map(|i| scene.normals[i as usize]);
    let texture = texture_index.map(|i| scene.texture_coordinates[i as usize]);
    let position = scene.vertices[position_index as usize];
    Self {
      position,
      normal,
      texture,
    }
  }
}

#[derive(Debug, Default)]
pub struct Model<'a> {
  pub vertices: Vec<Vertex>,
  pub name: String,
  pub material: Option<Material<'a>>,
}
impl<'a> Model<'a> {
  pub fn from_obj_model(obj_model: &ObjModel, scene: &Scene) -> Self {
    let name = obj_model.name.clone();
    let mut vertices = vec![];
    for obj_face in &obj_model.faces {
      for v in obj_face.vertices {
        vertices.push(Vertex::from_vertex_index(v, scene));
      }
    }

    // let material = obj_model.material.map(|n| {
    //   scene
    //     .materials
    //     .get_material_by_name(&name)
    //     .map(|obj_material| Material::from_obj_material(obj_material, scene))
    // });

    let material = if let Some(name) = &obj_model.material {
      if let Some(obj_material) = scene.materials.get_material_by_name(name) {
        Some(Material::from_obj_material(obj_material, scene))
      } else {
        None
      }
    } else {
      None
    };

    Self {
      name,
      vertices,
      material,
    }
  }
}

#[derive(Debug)]
pub struct Scene<'a> {
  pub models: Vec<Model<'a>>,
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub texture_coordinates: Vec<Vec2>,
  pub materials: Materials,
}

impl<'a> Scene<'a> {
  pub fn from_obj_scene(obj_scene: &mut ObjScene) -> Self {
    let materials = swap_and_move(&mut obj_scene.materials);
    let vertices = swap_and_move(&mut obj_scene.vertices);
    let normals = swap_and_move(&mut obj_scene.normals);
    let texture_coordinates = swap_and_move(&mut obj_scene.texture_coordinates);

    Self {
      models: Default::default(),
      vertices,
      normals,
      texture_coordinates,
      materials,
    }
  }
}

pub fn from_obj_path(relative_path: &str) -> Result<Scene, ParserError> {
  let mut parser = load_obj(relative_path)?;
  let obj_scene = parser.parse()?;
  {
    let mut scene = Scene::from_obj_scene(obj_scene);
    for obj_model in &obj_scene.models {
      scene.models.push(Model::from_obj_model(obj_model, &scene));
    }
    Ok(scene)
  }
}

use crate::{
  math::{Vec2, Vec3, Vec4},
  obj_loader::{
    defines::ParserError,
    load_obj,
    material::{Material as ObjMaterial, MaterialBase, Materials, Texture, TextureMap},
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

pub type Material<'a> = MaterialBase<TextureRefer<'a>>;

impl<'a> Material<'a> {
  pub fn from_obj_material(obj_material: &ObjMaterial, scene: &Scene) -> Self {
    let texture_map = TextureRefer::default();
    let name = obj_material.name.clone();
    let id = obj_material.id;

    Self::from_another_material_type(obj_material, name, texture_map, id)
  }
}

pub type VertexTexture = TextureMap<Vec4>;

#[derive(Debug, Clone, Copy)]
pub struct VertexMaterial {
  pub id: u32,
  pub textures: VertexTexture,
}

impl VertexMaterial {
  pub fn from_model_material(material: &Material, vt: &Vec2) -> Self {
    let textures = VertexTexture::from_another_texuture_map(&material.texture_map, |texture| {
      texture.get_pixel(*vt)
    });

    Self {
      id: material.id,
      textures,
    }
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
  pub position: Vec4,
  pub normal: Option<Vec3>,
  pub texture: Option<Vec2>,
  pub material: Option<VertexMaterial>,
}

impl Vertex {
  pub fn from_vertex_index(v: &VertexIndex, scene: &Scene, material: Option<&Material>) -> Self {
    let VertexIndex {
      position_index,
      normal_index,
      texture_index,
    } = v;

    let normal = normal_index.map(|i| scene.normals[i as usize]);
    let texture = texture_index.map(|i| scene.texture_coordinates[i as usize]);
    let position = Vec4::from_vec3(&scene.vertices[*position_index as usize], 1.0);

    let vertex_material = if let (Some(vt), Some(m)) = (texture, material) {
      Some(VertexMaterial::from_model_material(m, &vt))
    } else {
      None
    };

    Self {
      position,
      normal,
      texture,
      material: vertex_material,
    }
  }
}

#[derive(Debug, Default)]
pub struct Model<'a> {
  pub vertices: Vec<Vertex>,
  name: String,
  material: Option<Material<'a>>,
}
impl<'a> Model<'a> {
  pub fn from_obj_model(obj_model: &ObjModel, scene: &Scene) -> Self {
    let name = obj_model.name.clone();
    let mut vertices = vec![];

    let material: Option<Material<'a>> = if let Some(name) = &obj_model.material {
      if let Some(obj_material) = scene.materials.get_material_by_name(name) {
        Some(Material::from_obj_material(obj_material, scene))
      } else {
        None
      }
    } else {
      None
    };

    for obj_face in &obj_model.faces {
      for v in &obj_face.vertices {
        vertices.push(Vertex::from_vertex_index(v, scene, material.as_ref()));
      }
    }

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

  let mut scene = Scene::from_obj_scene(obj_scene);
  for obj_model in &obj_scene.models {
    scene.models.push(Model::from_obj_model(obj_model, &scene));
  }
  Ok(scene)
}

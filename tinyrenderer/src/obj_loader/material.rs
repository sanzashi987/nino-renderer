use image::{GenericImageView, ImageError};
use std::path::Path;
use std::{collections::HashMap, fmt::Debug};

use crate::math::{Vec2, Vec3, Vec4};

use super::defines::ParserError;

macro_rules! make_material_base {
  ($($prop:ident:$type:ty),+) => {

    #[derive(Debug, Default)]
    pub struct MaterialBase<Map: Default> {
      pub name: String,
      pub texture_map: Map,
      $(
        pub $prop: Option<$type>,
      )+
    }

    impl<Map: Default> MaterialBase<Map> {
      pub fn from_another_material_type<A: Default>(
        instance: &MaterialBase<A>,
        name: String,
        texture_map: Map,
      ) -> Self {
        Self {
          name,
          texture_map,
          $(
            $prop:instance.$prop,
          )+
        }
      }
    }
  };
}
make_material_base!(
  ambient: Vec3,
  diffuse: Vec3,
  specular: Vec3,
  emissive_coeficient: Vec3,
  specular_exponent: f32,
  dissolve: f32,
  transmission_filter: Vec3,
  optical_density: f32,
  illum: u8
);

pub type Material = MaterialBase<TexturePointer>;

macro_rules! make_texture_map {
  ($($prop:ident:$type:ty),+) => {
    #[derive(Debug, Default)]
    pub struct TextureMap<T> {
      $(
        pub $prop: Option<T>,
      )+
    }
    impl<T> TextureMap<T> {
      pub fn from_another_texuture_map<A, F: Fn(&A) -> T>(instance: &TextureMap<A>, f: F) -> Self {
        $(
          let $prop = if let Some(v) = &instance.$prop {
            Some(f(v))
          } else {
            None
          };
        )+
        Self {$($prop,)+ }
      }

      pub fn get_by_key(&self, key: &str) -> Option<&T> {
        match key {
          $(
            stringify!($prop) => self.$prop.as_ref(),
          )+
          _ => None,
        }
      }

    }

  };
}

make_texture_map!(
  ambient:std::any::Any,
  diffuse:std::any::Any,
  specular_color:std::any::Any,
  specular_highlight:std::any::Any,
  alpha:std::any::Any,
  refl:std::any::Any,
  bump:std::any::Any
);
pub type TexturePointer = TextureMap<String>;

#[derive(Debug, Default)]
pub struct Materials {
  last: Option<String>,
  materials: HashMap<String, Material>,
  textures: Textures,
}

impl Materials {
  pub fn new_material(&mut self, name: &str) {
    let name = name.to_string();
    let mut material = Material::default();
    material.name = name.clone();
    self.last = Some(name.clone());
    self.materials.insert(name, material);
  }

  pub fn get_material_by_name(&self, name: &str) -> Option<&Material> {
    self.materials.get(name)
  }

  pub fn get_mutates(&mut self) -> Result<(&mut Material, &mut Textures), ParserError> {
    let material = if let Some(name) = &self.last {
      self.materials.get_mut(name)
    } else {
      None
    };

    let material = material.ok_or(ParserError::MaterialNotFound)?;

    Ok((material, &mut self.textures))
  }
}

pub trait MoveMaterials {
  fn move_out_materials(&mut self) -> Materials;
  fn move_in_materials(&mut self, materials: Materials);
}

#[derive(Debug, Default)]
pub struct Mtl(pub Materials);

impl MoveMaterials for Mtl {
  fn move_out_materials(&mut self) -> Materials {
    std::mem::replace(&mut self.0, Default::default())
  }

  fn move_in_materials(&mut self, materials: Materials) {
    self.0 = materials;
  }
}

#[derive(Debug, Default)]
pub struct Texture {
  id: u32,
  name: String,
  image: Option<image::DynamicImage>,
  path: String,
}

impl Texture {
  pub fn load(name: &str, path: &Path, id: u32) -> Result<Self, ImageError> {
    let image_data = image::open(path).ok();

    Ok(Self {
      id,
      name: name.to_string(),
      image: image_data,
      path: path.to_str().expect("Not a valid texture path").to_string(),
    })
  }

  ///  @param vt standard vt with x,y range from -1 to 1.
  pub fn get_pixel(&mut self, vt: Vec2) -> Vec4 {
    if let None = self.image {
      self.image = image::open(std::path::Path::new(&self.path)).ok();
    }

    let img = self
      .image
      .as_ref()
      .expect(&format!("Fail to load texture:{}", &self.path.as_str()));

    let width = img.width();
    let height = img.height();

    let x = (vt.x * (width - 1) as f32) as u32;
    let y = (vt.y * (height - 1) as f32) as u32;

    let rgba = img.get_pixel(x, y).0;
    Vec4::new(
      rgba[0] as f32 / 255.0,
      rgba[1] as f32 / 255.0,
      rgba[2] as f32 / 255.0,
      rgba[3] as f32 / 255.0,
    )
  }
}

#[derive(Debug, Default)]
pub struct Textures {
  auto_incr_id: u32,
  data: HashMap<u32, Texture>,
  name_id_map: HashMap<String, u32>,
}

impl Textures {
  pub fn load(&mut self, filepath: &str, name: &str) -> Result<u32, ImageError> {
    if let Some(id) = self.name_id_map.get(name) {
      return Ok(*id);
    }

    let id = self.auto_incr_id;
    let path = Path::new(filepath);
    self.data.insert(id, Texture::load(name, path, id)?);
    self.name_id_map.insert(name.to_string(), id);
    self.auto_incr_id += 1;
    Ok(id)
  }

  pub fn get_texture_by_id(&self, id: u32) -> Option<&Texture> {
    self.data.get(&id)
  }
}

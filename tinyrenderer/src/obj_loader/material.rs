use image::{GenericImageView, ImageError};
use std::collections::HashMap;
use std::path::Path;

use crate::math::{Vec2, Vec3, Vec4};

use super::defines::{ParserError, ParserMode};
#[derive(Debug, Default)]
pub struct Material {
  pub name: String,
  pub ambient: Option<Vec3>,
  pub diffuse: Option<Vec3>,
  pub specular: Option<Vec3>,
  pub emissive_coeficient: Option<Vec3>,
  pub specular_exponent: Option<f32>,
  pub dissolve: Option<f32>,
  pub transmission_filter: Option<Vec3>,
  pub optical_density: Option<f32>,
  pub illum: Option<u8>,
  pub texture_map: TexturePointer,
}

#[derive(Debug, Default)]
pub struct TexturePointer {
  pub ambient: Option<String>,
  pub diffuse: Option<String>,
  pub specular_color: Option<String>,
  pub specular_highlight: Option<String>,
  pub alpha: Option<String>,
  pub refl: Option<String>,
  pub bump: Option<String>,
}

#[derive(Default)]
pub struct Materials {
  last: Option<String>,
  materials: HashMap<String, Material>,
  textures: Textures,
}

pub trait MoveTexutures {
  fn move_in_textures(&mut self, textures: Textures);
  fn move_out_textures(self) -> Textures;
}

impl Materials {
  pub fn new_material(&mut self, name: &str) {
    let name = name.to_string();
    let mut material = Material::default();
    material.name = name.clone();
    self.last = Some(name.clone());
    self.materials.insert(name, material);
  }

  pub fn get_current(&mut self) -> Result<&mut Material, ParserError> {
    let res = if let Some(name) = &self.last {
      self.materials.get_mut(name)
    } else {
      None
    };

    res.ok_or(ParserError::MaterialNotFound)
  }

  pub fn register_texture(filepath: String, name: String) {}
}

impl MoveTexutures for Materials {
  fn move_in_textures(&mut self, textures: Textures) {
    self.textures = textures;
  }

  fn move_out_textures(self) -> Textures {
    self.textures
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
  pub fn load(name: &str, path: &Path, id: u32, mode: ParserMode) -> Result<Self, ImageError> {
    let image_data = if let ParserMode::Lazy = mode {
      None
    } else {
      image::open(path).ok()
    };

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

    let img = self.image.as_ref().expect("Fail to load texture");

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
  pub fn load(&mut self, filepath: &Path, name: &str, mode: ParserMode) -> Result<u32, ImageError> {
    if let Some(id) = self.name_id_map.get(name) {
      return Ok(*id);
    }

    let id = self.auto_incr_id;
    self
      .data
      .insert(id, Texture::load(name, filepath, id, mode)?);
    self.name_id_map.insert(name.to_string(), id);
    self.auto_incr_id += 1;
    Ok(id)
  }

  pub fn get_texture_by_id(&self, id: u32) -> Option<&Texture> {
    self.data.get(&id)
  }
}

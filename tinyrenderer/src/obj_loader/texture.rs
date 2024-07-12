use std::{collections::HashMap, path::Path};

use image::{GenericImageView, ImageError};

use crate::math::{Vec2, Vec4};

#[derive(Debug)]
pub struct Texture {
  id: u32,
  name: String,
  image: image::DynamicImage,
}

impl Texture {
  pub fn load(name: &str, path: &Path, id: u32) -> Result<Self, ImageError> {
    let image_data = image::open(path)?;

    Ok(Self {
      id,
      name: name.to_string(),
      image: image_data,
    })
  }

  ///  @param vt standard vt with x,y range from -1 to 1.
  pub fn get_pixel(&self, vt: Vec2) -> Vec4 {
    let width = self.image.width();
    let height = self.image.height();

    let x = (vt.x * (width - 1) as f32) as u32;
    let y = (vt.y * (height - 1) as f32) as u32;

    let rgba = self.image.get_pixel(x, y).0;
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
  pub fn load(&mut self, filepath: &Path, name: &str) -> Result<u32, ImageError> {
    let id = self.auto_incr_id;
    self.data.insert(id, Texture::load(name, filepath, id)?);
    self.name_id_map.insert(name.to_string(), id);
    self.auto_incr_id += 1;
    Ok(id)
  }

  pub fn get_texture_by_id(&self, id: u32) -> Option<&Texture> {
    self.data.get(&id)
  }
}

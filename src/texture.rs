use std::collections::HashMap;

use image::{self, GenericImageView};

use crate::math::Vec4;

#[derive(Debug, Default)]
pub struct Texture {
  image: image::DynamicImage,
  id: u32,
  name: String,
}

impl Texture {
  pub fn load(filename: &str, id: u32, name: &str) -> image::ImageResult<Texture> {
    let img = image::open(filename)?;

    Ok(Self {
      image: img,
      id,
      name: name.to_string(),
    })
  }

  pub fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
    let pixel = self.image.get_pixel(x, y);
    let data = pixel.0;
    Vec4::new(
      data[0] as f32 / 255.0,
      data[1] as f32 / 255.0,
      data[2] as f32 / 255.0,
      data[3] as f32 / 255.0,
    )
  }

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn width(&self) -> u32 {
    self.image.width()
  }

  pub fn height(&self) -> u32 {
    self.image.height()
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}

#[derive(Debug, Default)]
pub struct TextureStore {
  auto_incre_id: u32,
  images: HashMap<u32, Texture>,
  name_id_map: HashMap<String, u32>,
}

impl TextureStore {
  pub fn load(&mut self, filename: &str, name: &str) -> image::ImageResult<u32> {
    let id = self.auto_incre_id;
    self.images.insert(id, Texture::load(filename, id, name)?);
    self.name_id_map.insert(name.to_string(), id);
    self.auto_incre_id += 1;
    Ok(id)
  }

  pub fn get_by_id(&self, id: u32) -> Option<&Texture> {
    self.images.get(&id)
  }

  pub fn get_by_name(&self, name: &str) -> Option<&Texture> {
    let id = self.get_id(name)?;
    self.get_by_id(*id)
  }

  pub fn get_id(&self, name: &str) -> Option<&u32> {
    self.name_id_map.get(&name.to_string())
  }
}

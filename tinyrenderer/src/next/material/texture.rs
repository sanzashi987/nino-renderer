use std::path::Path;

use image::{open, DynamicImage, GenericImageView, ImageError};

use crate::math::{Vec2, Vec4};

#[derive(Debug, Default)]
pub struct Texture {
  id: u32,
  name: String,
  image: Option<DynamicImage>,
  path: String,
}

impl Texture {
  pub fn load(name: &str, path: &Path, id: u32) -> Result<Self, ImageError> {
    let image_data = open(path).ok();

    Ok(Self {
      id,
      name: name.to_string(),
      image: image_data,
      path: path.to_str().expect("Not a valid texture path").to_string(),
    })
  }

  ///  @param vt standard vt with x,y range from -1 to 1.
  pub fn get_pixel(&self, vt: Vec2) -> Vec4 {
    let img = self
      .image
      .as_ref()
      .expect(&format!("Fail to load texture:{}", &self.path.as_str()));

    let width = img.width();
    let height = img.height();

    let x = (vt.x * (width - 1) as f32) as u32;
    let y = ((1.0 - vt.y) * (height - 1) as f32) as u32;

    let rgba = img.get_pixel(x, y).0;
    Vec4::new(
      rgba[0] as f32 / 255.0,
      rgba[1] as f32 / 255.0,
      rgba[2] as f32 / 255.0,
      rgba[3] as f32 / 255.0,
    )
  }
}

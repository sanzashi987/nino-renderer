use std::fmt::format;

use image::{open, DynamicImage, GenericImageView, ImageError};

use crate::math::{Vec2, Vec4};

#[derive(Debug)]
pub enum Filter {
  Nearest,
  NearestMipmapNearest,
  NearestMipMapNearest,
  NearestMipmapLinear,
  NearestMipMapLinear,
  Linear,
  LinearMipmapNearest,
  LinearMipMapNearest,
  LinearMipmapLinear,
  LinearMipMapLinear,
}

#[derive(Debug)]
pub struct Texture {
  pub id: u32,
  image: DynamicImage,
  pub path: String,
  pub name: String,
  pub min_filter: Filter,
  pub mag_filter: Filter,
}

impl Default for Texture {
  fn default() -> Self {
    let image = DynamicImage::new(1, 1, image::ColorType::Rgb8);

    Self {
      id: 0,
      image,
      path: Default::default(),
      name: Default::default(),
      min_filter: Filter::Linear,
      mag_filter: Filter::LinearMipmapLinear,
    }
  }
}

impl Texture {
  pub fn new(w: u32, h: u32) -> Self {
    let mut instance = Self::default();
    instance.image = DynamicImage::new(w, h, image::ColorType::Rgb16);
    instance
  }

  pub fn load(path: &str, id: u32) -> Result<Self, ImageError> {
    let image = open(path)?;
    let mut instance = Self::default();

    (instance.id, instance.path, instance.image) = (id, path.to_string(), image);

    Ok(instance)
  }

  ///  @param uv standard uv with x,y range from -1 to 1.
  pub fn get_pixel(&self, uv: Vec2) -> Vec4 {
    let image = &self.image;

    let width = image.width();
    let height = image.height();

    let x = (uv.x * (width - 1) as f32) as u32;
    let y = ((1.0 - uv.y) * (height - 1) as f32) as u32;

    let rgba = image.get_pixel(x, y).0;
    Vec4::new(
      rgba[0] as f32 / 255.0,
      rgba[1] as f32 / 255.0,
      rgba[2] as f32 / 255.0,
      rgba[3] as f32 / 255.0,
    )
  }
}

pub fn texture_2D(sampler: &Texture, uv: Vec2) -> Vec4 {
  let img = &sampler.image;

  let width = img.width();
  let height = img.height();

  let x = (uv.x * (width - 1) as f32) as u32;
  let y = ((1.0 - uv.y) * (height - 1) as f32) as u32;
  let rgba = img.get_pixel(x, y).0;
  Vec4::new(
    rgba[0] as f32 / 255.0,
    rgba[1] as f32 / 255.0,
    rgba[2] as f32 / 255.0,
    rgba[3] as f32 / 255.0,
  )
}

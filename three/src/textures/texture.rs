use std::sync::Mutex;

use image::{open, DynamicImage, GenericImage, GenericImageView, ImageError, Rgba};

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

static GLOBAL_ID: Mutex<u32> = Mutex::new(0);

#[derive(Debug)]
pub struct Texture {
  pub id: u32,
  image: DynamicImage,
  pub path: String,
  pub name: String,
  pub min_filter: Filter,
  pub mag_filter: Filter,

  bit_depth: image::ColorType,
}

impl Default for Texture {
  fn default() -> Self {
    let image = DynamicImage::new(1, 1, image::ColorType::Rgb8);
    let mut id = GLOBAL_ID.lock().unwrap();
    let current_id = *id;
    *id = current_id + 1;
    Self {
      id: current_id,
      image,
      path: Default::default(),
      name: Default::default(),
      min_filter: Filter::Linear,
      mag_filter: Filter::LinearMipmapLinear,
      bit_depth: image::ColorType::Rgb8,
    }
  }
}

impl Texture {
  pub fn new(w: u32, h: u32) -> Self {
    let mut instance = Self::default();
    instance.image = DynamicImage::new(w, h, instance.bit_depth);
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

  pub fn write(&mut self, x: u32, y: u32, color: Vec4) {
    let pixel = Rgba([color.x as u8, color.y as u8, color.z as u8, color.w as u8]);
    self.image.put_pixel(x, y, pixel);
  }

  pub fn set_size(&mut self, w: u32, h: u32) {
    self.image = DynamicImage::new(w, h, self.bit_depth);
  }

  pub fn take_color(&mut self) -> (Vec<u8>, image::ColorType) {
    let image = &self.image;
    let width = image.width();
    let height = image.height();
    let empty_image = DynamicImage::new(width, height, self.bit_depth);
    let res = std::mem::replace(&mut self.image, empty_image);
    (res.into_bytes(), self.bit_depth)
  }
}

pub fn texture_2d(sampler: &Texture, uv: Vec2) -> Vec4 {
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

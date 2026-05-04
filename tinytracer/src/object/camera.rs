use std::usize;

use math::{Vec2, Vec3};

use super::{
  ray::{Hittable, Ray},
  world::World,
};

#[derive(Debug, Default)]
pub struct Camera {
  pub aspect_ratio: f32,
  pub center: Vec3,
  pub image_width: i32,
  pub samples_per_pixel: i32,
  pub max_depth: i32,

  image_height: i32,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
  pixel00_loc: Vec3,
}

impl Camera {
  pub fn image_height(&self) -> i32 {
    self.image_height
  }

  pub fn new(image_width: i32, aspect_ratio: f32, center: Vec3) -> Self {
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0f32;
    let viewport_height = 2.0f32;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

    // see ![viewport](../images/viewport.jpg)
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    let viewport_upper_left =
      center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    Self {
      image_width,
      image_height,
      aspect_ratio,
      center,
      pixel_delta_u,
      pixel_delta_v,
      pixel00_loc,
      ..Default::default()
    }
  }

  pub fn ray_color(world: &World, ray: &Ray) -> Vec3 {
    if let Some(rec) = world.hit(ray, None) {
      return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_dir = ray.direction.normalize();
    let a = 0.5 * (unit_dir.y + 1.0);

    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
  }

  pub fn get_ray(&self, i: usize, j: usize) -> Ray {
    let Self {
      center,
      pixel_delta_u,
      pixel_delta_v,
      pixel00_loc,
      ..
    } = self;
    let mut offset = Vec3::random() - Vec3::new(0.5, 0.5, 0.5);
    offset.z = 0.0;

    let pixel_center = *pixel00_loc
      + (*pixel_delta_u * (i as f32 + offset.x))
      + (*pixel_delta_v * (j as f32 + offset.y));
    let ray_direction = pixel_center - *center;

    Ray::new(pixel_center, ray_direction)
  }

  pub fn render(&self, world: &World, buffer: &mut Vec<u8>) {
    let Self {
      image_width,
      image_height,
      pixel00_loc,
      pixel_delta_u,
      pixel_delta_v,
      center,
      ..
    } = self;
    for j in 0..*image_height {
      for i in 0..*image_width {
        let pixel_center =
          *pixel00_loc + (*pixel_delta_u * (i as f32)) + (*pixel_delta_v * (j as f32));
        let ray_direction = pixel_center - *center;

        let r = Ray::new(*center, ray_direction);
        let color = Self::ray_color(world, &r) * 255.0;

        let idx = (j as usize * *image_width as usize + i as usize) * 3;
        buffer[idx] = (color.x as u8).min(255);
        buffer[idx + 1] = (color.y as u8).min(255);
        buffer[idx + 2] = (color.z as u8).min(255);
      }
    }
  }
}

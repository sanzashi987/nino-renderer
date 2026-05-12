use std::usize;

use math::Vec3;

use super::{
  ray::{Hittable, Ray},
  world::World,
};

fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
  let on_unit_sphere = Vec3::random_unit();
  if on_unit_sphere * *normal > 0.0 {
    on_unit_sphere
  } else {
    on_unit_sphere * -1.0
  }
}

pub fn ray_color(world: &World, ray: &Ray, depth: i32) -> Vec3 {
  if depth == 0 {
    return Vec3::zero();
  }

  if let Some(rec) = world.hit(ray, None) {
    let direction = Vec3::random_unit() + rec.normal;
    return ray_color(world, &Ray::new(rec.point, direction), depth - 1) * 0.1;
  }

  let unit_dir = ray.direction.normalize();
  let a = 0.5 * (unit_dir.y + 1.0);

  return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
}

fn gamma_correction(linear: f32) -> f32 {
  if linear > 0 {
    linear.sqrt()
  } else {
    0
  }
}

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
  pixel_samples_scale: f32,
}

impl Camera {
  pub fn image_height(&self) -> i32 {
    self.image_height
  }

  pub fn new(image_width: i32, aspect_ratio: f32, center: Vec3) -> Self {
    let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);

    let focal_length = 1.0f32;
    let samples_per_pixel = 100;
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
      samples_per_pixel,
      pixel_samples_scale: 1.0 / samples_per_pixel as f32,
      max_depth: 10,
      ..Default::default()
    }
  }

  pub fn get_ray(&self, i: i32, j: i32) -> Ray {
    let Self {
      center,
      pixel_delta_u,
      pixel_delta_v,
      pixel00_loc,
      ..
    } = self;
    let mut offset = Vec3::random() - Vec3::new(0.5, 0.5, 0.5);
    // let mut offset = Vec3::zero();
    offset.z = 0.0;

    let pixel_center = *pixel00_loc
      + (*pixel_delta_u * (i as f32 + offset.x))
      + (*pixel_delta_v * (j as f32 + offset.y));
    let ray_direction = pixel_center - *center;

    Ray::new(*center, ray_direction)
  }

  pub fn render(&self, world: &World, buffer: &mut Vec<u8>) {
    let Self {
      image_width,
      image_height,
      samples_per_pixel,
      max_depth,
      ..
    } = self;
    for j in 0..*image_height {
      for i in 0..*image_width {
        let mut c = Vec3::zero();

        for _ in 0..*samples_per_pixel {
          let r = self.get_ray(i, j);
          let color = ray_color(world, &r, *max_depth);
          c += color;
        }

        let idx = (j as usize * *image_width as usize + i as usize) * 3;
        c *= self.pixel_samples_scale;
        let x = gamma_correction(c.x);
        let y = gamma_correction(c.y);
        let z = gamma_correction(c.z);

        buffer[idx] = (x.clamp(0.0, 1.0) * 255.0) as u8;
        buffer[idx + 1] = (y.clamp(0.0, 1.0) * 255.0) as u8;
        buffer[idx + 2] = (z.clamp(0.0, 1.0) * 255.0) as u8;
      }
    }
  }
}

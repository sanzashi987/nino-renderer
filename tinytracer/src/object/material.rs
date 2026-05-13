use std::sync::Arc;

use math::Vec3;

use crate::object::ray::{HitRecord, Ray};

pub trait Material {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambertian {
  pub albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Arc<Self> {
    let instance = Self { albedo };
    Arc::new(instance)
  }
}

impl Material for Lambertian {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    let mut scatter_dir = rec.normal + Vec3::random_unit();

    if scatter_dir.near_zero() {
      scatter_dir = rec.normal
    }

    let ray = Ray::new(rec.point, scatter_dir);
    Some((self.albedo, ray))
  }
}

pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f32,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f32) -> Arc<Self> {
    let instance = Self {
      albedo,
      fuzz: fuzz.min(1.0),
    };
    Arc::new(instance)
  }
}

impl Material for Metal {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    let mut reflected = ray_in.direction.reflect(&rec.normal);
    reflected = reflected.normalize() + Vec3::random_unit() * self.fuzz;
    let scattered = Ray::new(rec.point, reflected);
    if scattered.direction * rec.normal > 0.0 {
      Some((self.albedo, scattered))
    } else {
      None
    }
  }
}

pub struct Dielectric {
  pub refraction_index: f32,
}

impl Dielectric {
  pub fn new(refraction_index: f32) -> Arc<Self> {
    let instance = Self { refraction_index };
    Arc::new(instance)
  }
}

impl Material for Dielectric {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    let attenuation = Vec3::new(1.0, 1.0, 1.0);

    let ri = if rec.front_face {
      1.0 / self.refraction_index
    } else {
      self.refraction_index
    };

    let cos_theta = (ray_in.direction * -1.0 * rec.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

    let refracted = if ri * sin_theta > 1.0 {
      // cannot refract , do reflect
      ray_in.direction.reflect(&rec.normal)
    } else {
      ray_in.direction.refract(&rec.normal, ri)
    };

    // let refracted = ray_in.direction.refract(&rec.normal, ri);
    Some((attenuation, Ray::new(rec.point, refracted)))
  }
}

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
    let s = Self { albedo };
    Arc::new(s)
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
}

impl Metal {
  pub fn new(albedo: Vec3) -> Arc<Self> {
    let s = Self { albedo };
    Arc::new(s)
  }
}

impl Material for Metal {
  fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    let reflected = ray_in.direction.reflect(&rec.normal);
    let scattered = Ray::new(rec.point, reflected);
    Some((self.albedo, scattered))
  }
}

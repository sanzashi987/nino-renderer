use math::Vec3;

use crate::object::ray::{HitRecord, Ray};

pub trait Material {
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambertian {
  pub diffuse: Vec3,
  pub albedo: Vec3,
  pub shininess: f32,
}

impl Lambertian {
  pub fn new(diffuse: Vec3, albedo: Vec3, shininess: f32) -> Self {
    Self {
      diffuse,
      albedo,
      shininess,
    }
  }
}

impl Material for Lambertian {
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
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

impl Material for Metal {
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    None
  }
}

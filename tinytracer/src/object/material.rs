use math::{Vec2, Vec3};

use crate::object::ray::{HitRecord, Ray};

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambertian {
  pub diffuse: Vec3,
  pub albedo: Vec2,
  pub shininess: f32,
}

impl Lambertian {
  pub fn new(diffuse: Vec3, albedo: Vec2, shininess: f32) -> Self {
    Self {
      diffuse,
      albedo,
      shininess,
    }
  }
}

impl Material for Lambertian {
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    None
  }
}

pub trait Material {
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

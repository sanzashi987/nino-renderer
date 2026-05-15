use core::f32;
use std::sync::Arc;

use math::Vec3;

use crate::object::ray::{HitConfig, HitRecord, Hittable, Ray};

use super::material::Material;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub material: Arc<dyn Material>,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
    Self {
      center,
      radius,
      material,
    }
  }

  pub fn ray_intersect(
    &self,
    ray_origin: &Vec3,
    ray_dir: &Vec3,
    t_min: f32,
    t_max: f32,
  ) -> Option<f32> {
    let to_center = self.center - *ray_origin;
    let to_center_projected_at_dir = to_center * *ray_dir; // dot product
    let ray_to_center =
      to_center * to_center - to_center_projected_at_dir * to_center_projected_at_dir;
    let radius_exq = self.radius.powi(2);
    if ray_to_center > radius_exq {
      return None;
    }

    let half_chord = (radius_exq - ray_to_center).sqrt();

    let mut first_intersect = to_center_projected_at_dir - half_chord;
    if first_intersect <= t_min || first_intersect >= t_max {
      first_intersect = to_center_projected_at_dir + half_chord;
      if first_intersect <= t_min || first_intersect >= t_max {
        return None;
      }
    }

    Some(first_intersect)
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, config: Option<HitConfig>) -> Option<HitRecord> {
    let HitConfig { t_min, t_max } = config.unwrap_or(HitConfig::default());

    if let Some(t) = self.ray_intersect(&ray.origin, &ray.direction, t_min, t_max) {
      let normal = (ray.at(t) - self.center).normalize();
      return Some(HitRecord::new(ray, normal, t, self.material.clone()));
    }
    None
  }
}

use core::f32;

use math::Vec3;

use crate::object::ray::{HitConfig, HitRecord, Hittable, Ray};

use super::material::Material;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub material: Material,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
    Self {
      center,
      radius,
      material,
    }
  }

  /**
   * @return the distance to the ray origin when first intersection occurs
   */
  pub fn ray_intersect(&self, ray_origin: &Vec3, ray_dir: &Vec3) -> Option<f32> {
    let to_center = self.center - *ray_origin;
    let to_center_projected_at_dir = to_center * *ray_dir; // dot product
    let ray_to_center =
      to_center * to_center - to_center_projected_at_dir * to_center_projected_at_dir;
    let raduis_exq = self.radius.powi(2);
    if ray_to_center > raduis_exq {
      return None;
    }

    let half_chord = (raduis_exq - ray_to_center).sqrt();

    let mut first_intersect = to_center_projected_at_dir - half_chord;
    let far_intersect = to_center_projected_at_dir + half_chord;

    if first_intersect < 0.0 {
      // emit inside the sphere
      first_intersect = far_intersect;
    }

    if first_intersect < 0.0 {
      return None;
    }

    return Some(first_intersect);
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, config: Option<HitConfig>) -> Option<HitRecord> {
    let HitConfig { t_min, t_max } = config.unwrap_or(HitConfig {
      t_min: 0.001,
      t_max: f32::INFINITY,
    });

    if let Some(t) = self.ray_intersect(&ray.origin, &ray.direction) {
      if t <= t_min || t >= t_max {
        return None;
      }

      let normal = (ray.at(t) - self.center).normalize();
      let point = ray.at(t);
      return Some(HitRecord { point, normal, t });
    }
    return None;
  }
}

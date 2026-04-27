use math::Vec3;

pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Self {
    let dir = direction.normalize();
    Self {
      origin,
      direction: dir,
    }
  }

  pub fn at(&self, t: f32) -> Vec3 {
    return self.origin + self.direction * t;
  }
}

pub struct HitConfig {
  pub t_min: f32,
  pub t_max: f32,
}

pub struct HitRecord {
  pub point: Vec3,
  pub normal: Vec3,
  pub t: f32,
}
pub trait Hittable {
  fn hit(&self, ray: &Ray, config: Option<HitConfig>) -> Option<HitRecord>;
}

// pub struct RayColor

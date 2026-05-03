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
#[derive(Debug, Clone, Copy)]
pub struct HitConfig {
  pub t_min: f32,
  pub t_max: f32,
}

impl HitConfig {
  pub fn size(&self) -> f32 {
    self.t_max - self.t_min
  }
  pub fn containes(&self, x: f32) -> bool {
    x >= self.t_min && x <= self.t_max
  }
  pub fn surrounds(&self, x: f32) -> bool {
    self.t_min < x && x < self.t_max
  }
}

impl Default for HitConfig {
  fn default() -> Self {
    Self {
      t_min: 0.001,
      t_max: f32::INFINITY,
    }
  }
}

pub struct HitRecord {
  pub point: Vec3,
  pub normal: Vec3,
  pub t: f32,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new(ray: &Ray, out_normal: Vec3, t: f32) -> Self {
    let mut h = Self {
      point: ray.at(t),
      t,
      normal: out_normal,
      front_face: true,
    };

    h.set_face_normal(ray, out_normal);
    h
  }

  fn set_face_normal(&mut self, ray: &Ray, out_normal: Vec3) {
    let front_face = ray.direction * out_normal < 0.0;
    self.front_face = front_face;
    self.normal = if front_face {
      out_normal
    } else {
      out_normal * -1.0
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, config: Option<HitConfig>) -> Option<HitRecord>;
}

// pub struct RayColor

use math::Vec3;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Self {
    Self { center, radius }
  }

  pub fn ray_intersect(&self, ray_orig: &Vec3) -> bool {

  }
}

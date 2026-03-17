use math::{Vec2, Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Material {
  pub diffuse: Vec3,
  pub albedo: Vec2,
  pub shininess: f32,
}
impl Material {
  pub fn new(diffuse: Vec3, albedo: Vec2, shininess: f32) -> Self {
    Self {
      diffuse,
      albedo,
      shininess,
    }
  }
}

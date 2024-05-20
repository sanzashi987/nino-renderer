use super::Vec2;
pub struct Barycentric {
  alpha: f32,
  beta: f32,
  gamma: f32,
}

impl Barycentric {
  pub fn new(pt: &Vec2, triangle: &[Vec2; 3]) -> Self {}

  pub fn is_valid(&self) -> bool {
    self.alpha + self.beta + self.gamma < 1.00001
  }
  pub fn alpha(&self) -> f32 {
    self.alpha
  }
  pub fn beta(&self) -> f32 {
    self.beta
  }
  pub fn gamma(&self) -> f32 {
    self.gamma
  }
}

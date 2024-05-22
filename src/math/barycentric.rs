use super::Vec2;
pub struct Barycentric {
  alpha: f32,
  beta: f32,
  gamma: f32,
}

impl Barycentric {
  pub fn new(pt: &Vec2, triangle: &[Vec2; 3]) -> Self {
    let area_twice = (triangle[1] - triangle[0]).cross(&(triangle[2] - triangle[1]));
    let alpha = ((triangle[1] - *pt).cross(&(triangle[2] - *pt)) / area_twice).abs();
    let beta = ((triangle[0] - *pt).cross(&(triangle[2] - *pt)) / area_twice).abs();
    let gamma = ((triangle[0] - *pt).cross(&(triangle[1] - *pt)) / area_twice).abs();
    Self { alpha, beta, gamma }
  }

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

use super::Vec2;

pub struct Barycentric {
  alpha: f32,
  beta: f32,
  gamma: f32,
}

fn triangle_area(A: &Vec2, B: &Vec2, C: &Vec2) -> f32 {
  (*B - *A).cross(&(*C - *A)).abs() / 2.0
}

impl Barycentric {
  pub fn new(pt: &Vec2, pts: &[Vec2; 3]) -> Self {
    let [a, b, c] = pts;

    let s_abc = triangle_area(a, b, c);
    let s_pbc = triangle_area(pt, b, c);
    let s_apc = triangle_area(a, pt, c);
    let s_abp = triangle_area(a, b, pt);

    let alpha = s_pbc / s_abc;
    let beta = s_apc / s_abc;
    let gamma = s_abp / s_abc;

    Self { alpha, beta, gamma }
  }

  pub fn is_inside(&self) -> bool {
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

use std::ops::{Add, Mul, Neg, Sub};
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub fn dot(self, rhs: Vec2) -> f32 {
    self.x * rhs.x + self.y * rhs.y
  }
}

impl Add<Vec2> for Vec2 {
  type Output = Self;

  fn add(self, rhs: Vec2) -> Self {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl Sub<Vec2> for Vec2 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}
impl Mul<f32> for Vec2 {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

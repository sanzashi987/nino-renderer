use std::ops::{Div, Mul};

use super::{Mat3, Mat4};
#[derive(Debug, PartialEq)]
pub struct Quaternion {
  /// q = w + xi + yj + zk
  w: f32,
  x: f32,
  y: f32,
  z: f32,
}

impl Mul<f32> for Quaternion {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self::new(rhs * self.w, rhs * self.x, rhs * self.y, rhs * self.z)
  }
}
impl Mul<Quaternion> for Quaternion {
  type Output = Self;

  fn mul(self, rhs: Quaternion) -> Self::Output {
    let x = self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y;
    let y = self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z;
    let z = self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x;
    let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;

    Self::new(w, x, y, z)
  }
}

impl Div<f32> for Quaternion {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    self * (1.0 / rhs)
  }
}

impl Quaternion {
  pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
    Self { w, x, y, z }
  }

  pub fn length_square(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
  }

  pub fn length(&self) -> f32 {
    self.length().sqrt()
  }

  pub fn dot(&self, another: Self) -> f32 {
    self.x * another.x + self.y * another.y + self.z * another.z + self.w * another.w
  }

  pub fn conjugate(&self) -> Self {
    Self::new(-self.x, -self.y, -self.z, self.w)
  }

  pub fn inverse(&self) -> Self {
    self.conjugate() / self.length_square()
  }

  pub fn identity() -> Self {
    Self::new(1.0, 0.0, 0.0, 0.0)
  }
}

impl Quaternion {
  ///     [ 1 - 2(y^2 + z^2),    2(xy - wz)   ,    2(xz + wy)    ]
  /// R = [    2(xy + wz)   , 1 - 2(x^2 + z^2),    2(yz - wx)    ]
  ///     [    2(xz - wy)   ,    2(yz + wx)   , 1 - 2(x^2 + y^2) ]
  #[rustfmt::skip]
  pub fn make_rotate_matrix(&self) -> Mat4 {
    let Self { w, x, y, z } = self;
    Mat4::from_row(&[
      1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - w * z), 2.0 * (x * z + w * y), 0.0,
      2.0 * (x * y + w * z), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - w * x), 0.0,
      2.0 * (x * z - w * y), 2.0 * (y * z + w * x), 1.0 - 2.0 * (x * x + y * y), 0.0,
      0.0,                    0.0,                   0.0,                        1.0,
    ])
  }
}

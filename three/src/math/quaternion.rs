use std::ops::{Div, Mul};

use super::{Mat4, Vec4};
#[derive(Debug, PartialEq, Clone, Copy)]
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
    self.length_square().sqrt()
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

  pub fn make_rotate_matrix(&self) -> Mat4 {
    let Self { w, x, y, z } = self;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);

    let mut res = Mat4::identity();

    res.set_col(0, Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0));
    res.set_col(1, Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0));
    res.set_col(2, Vec4::new(xz + wy, yz - wx, xx + yy, 0.0));

    res
  }

  pub fn update_from_rotate_matrix(&mut self, mat: Mat4) {
    let q: Self = mat.into();
    (self.w, self.x, self.y, self.z) = (q.w, q.x, q.y, q.z)
  }
}

impl From<Mat4> for Quaternion {
  fn from(value: Mat4) -> Self {
    let Vec4 {
      x: m11,
      y: m21,
      z: m31,
      ..
    } = value.get_col(0);
    let Vec4 {
      x: m12,
      y: m22,
      z: m32,
      ..
    } = value.get_col(1);
    let Vec4 {
      x: m13,
      y: m23,
      z: m33,
      ..
    } = value.get_col(2);
    let trace = m11 + m22 + m33;
    let (mut w, mut x, mut y, mut z) = (0.0, 0.0, 0.0, 0.0);
    if trace > 0.0 {
      let s = 0.5 / (trace + 1.0).sqrt();
      w = 0.25 / s;
      x = (m32 - m23) * s;
      y = (m13 - m31) * s;
      z = (m21 - m12) * s;
    } else {
      let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();

      w = (m21 - m12) / s;
      x = (m13 + m31) / s;
      y = (m23 + m32) / s;
      z = 0.25 * s;
    };
    Self { w, x, y, z }
  }
}

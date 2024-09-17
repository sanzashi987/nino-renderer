use super::{Vec3, Vec4};
use std::ops::{Add, Div, Mul};

macro_rules! define_mat {
  ($name:ident, $dim:expr) => {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct $name {
      data: [f32; $dim * $dim],
    }

    impl $name {
      pub fn from_row(data: &[f32; $dim * $dim]) -> Self {
        Self { data: data.clone() }
      }

      pub fn from_col(data: &[f32; $dim * $dim]) -> Self {
        let mut mat = $name::zeros();
        for x in 0..$dim {
          for y in 0..$dim {
            mat.set(x, y, data[y + $dim * x]);
          }
        }
        mat
      }

      pub fn zeros() -> Self {
        Self {
          data: [0.0; $dim * $dim],
        }
      }
      pub fn ones() -> Self {
        Self {
          data: [1.0; $dim * $dim],
        }
      }

      pub fn identity() -> Self {
        let mut mat = $name::zeros();
        for i in 0..$dim {
          mat.set(i, i, 1.0);
        }
        mat
      }

      pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[x + y * $dim]
      }

      pub fn set(&mut self, x: usize, y: usize, value: f32) {
        self.data[x + y * $dim] = value;
      }

      pub fn transpose(&self) -> Self {
        let mut result = Self::identity();
        for x in 0..$dim {
          for y in 0..$dim {
            result.set(y, x, self.get(x, y));
          }
        }
        result
      }
    }
    impl Mul for $name {
      type Output = Self;
      fn mul(self, rhs: Self) -> Self::Output {
        let mut mat = $name::zeros();

        for y in 0..$dim {
          for x in 0..$dim {
            let mut sum = 0.0;
            for d in 0..$dim {
              sum += self.get(d, y) * rhs.get(x, d);
            }
            mat.set(x, y, sum);
          }
        }
        mat
      }
    }

    impl Mul<f32> for $name {
      type Output = Self;
      fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = $name::zeros();
        for x in 0..$dim {
          for y in 0..$dim {
            mat.set(x, y, self.get(x, y) * rhs);
          }
        }
        mat
      }
    }

    impl Div<f32> for $name {
      type Output = Self;
      fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
      }
    }

    impl Add for $name {
      type Output = Self;
      fn add(self, rhs: Self) -> Self::Output {
        let mut mat = $name::zeros();
        for y in 0..$dim {
          for x in 0..$dim {
            let sum = self.get(x, y) + rhs.get(x, y);
            mat.set(x, y, sum)
          }
        }
        mat
      }
    }

    impl PartialEq for $name {
      fn eq(&self, other: &Self) -> bool {
        self.data == other.data
      }
    }
  };
}

define_mat!(Mat2, 2);
define_mat!(Mat3, 3);
define_mat!(Mat4, 4);

impl Mul<Vec4> for Mat4 {
  type Output = Vec4;

  fn mul(self, rhs: Vec4) -> Self::Output {
    Vec4::new(
      self.get(0, 0) * rhs.x
        + self.get(1, 0) * rhs.y
        + self.get(2, 0) * rhs.z
        + self.get(3, 0) * rhs.w,
      self.get(0, 1) * rhs.x
        + self.get(1, 1) * rhs.y
        + self.get(2, 1) * rhs.z
        + self.get(3, 1) * rhs.w,
      self.get(0, 2) * rhs.x
        + self.get(1, 2) * rhs.y
        + self.get(2, 2) * rhs.z
        + self.get(3, 2) * rhs.w,
      self.get(0, 3) * rhs.x
        + self.get(1, 3) * rhs.y
        + self.get(2, 3) * rhs.z
        + self.get(3, 3) * rhs.w,
    )
  }
}

impl Mat2 {
  pub fn det(&self) -> f32 {
    self.get(0, 0) * self.get(1, 1) - self.get(1, 0) * self.get(0, 1)
  }

  #[rustfmt::skip]
  pub fn inverse(&self) -> Option<Self> {
    let d = self.det();
    if d.abs() <= f32::EPSILON {
      return None;
    }
    Some(Mat2::from_row(&[
      self.get(1, 1) / d, -self.get(1, 0) / d,
      -self.get(0, 1) / d, self.get(0, 0) / d
    ]))
  }
}

impl Mat3 {
  #[rustfmt::skip]
  pub fn det(&self) -> f32 {
    self.get(0, 0) * self.get(1, 1) * self.get(2, 2)
      + self.get(2, 0) * self.get(0, 1) * self.get(1, 2)
      + self.get(1, 0) * self.get(2, 1) * self.get(0, 2)
      - (self.get(2, 0) * self.get(1, 1) * self.get(0, 2)
          + self.get(1, 0) * self.get(0, 1) * self.get(2, 2)
          + self.get(0, 0) * self.get(1, 2) * self.get(2, 1))
    }

  #[rustfmt::skip]
  pub fn inverse(&self) -> Option<Self> {
    let d = self.det();
    if d.abs() <= f32::EPSILON {
      return None;
    }
    Some(Mat3::from_row(&[
      self.get(1, 1) * self.get(2, 2) - self.get(2, 1) * self.get(1, 2),
      self.get(2, 0) * self.get(1, 2) - self.get(1, 0) * self.get(2, 2),
      self.get(1, 0) * self.get(2, 1) - self.get(2, 0) * self.get(1, 1),
      self.get(2, 1) * self.get(0, 2) - self.get(0, 1) * self.get(2, 2),
      self.get(0, 0) * self.get(2, 2) - self.get(2, 0) * self.get(0, 2),
      self.get(0, 1) * self.get(2, 1) - self.get(0, 0) * self.get(2, 0),
      self.get(0, 1) * self.get(1, 2) - self.get(1, 1) * self.get(0, 2),
      self.get(1, 0) * self.get(0, 2) - self.get(0, 0) * self.get(1, 2),
      self.get(0, 0) * self.get(1, 1) - self.get(1, 0) * self.get(0, 1),
    ]) / d)
  }
}

impl Mul<Vec3> for Mat3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    Vec3::new(
      self.get(0, 0) * rhs.x + self.get(1, 0) * rhs.y + self.get(2, 0) * rhs.z,
      self.get(0, 1) * rhs.x + self.get(1, 1) * rhs.y + self.get(2, 1) * rhs.z,
      self.get(0, 2) * rhs.x + self.get(1, 2) * rhs.y + self.get(2, 2) * rhs.z,
    )
  }
}

impl Mat3 {
  pub fn get_col(&self, x: usize) -> Vec3 {
    Vec3::new(self.get(x, 0), self.get(x, 1), self.get(x, 2))
  }

  pub fn set_col(&mut self, x: usize, column: Vec3) {
    self.set(x, 0, column.x);
    self.set(x, 1, column.y);
    self.set(x, 2, column.z);
  }

  pub fn get_orthogonal_basis(eye: Vec3, target: Vec3, up: Vec3) -> Self {
    let z = (eye - target).normalize();
    let x = up.cross(&z).normalize();
    let y = z.cross(&x).normalize();
    Self::from_row(&[x.x, x.y, x.z, y.x, y.y, y.z, z.x, z.y, z.z])
  }
}

impl Mat4 {
  pub fn set_col(&mut self, x: usize, column: Vec4) {
    self.set(x, 0, column.x);
    self.set(x, 1, column.y);
    self.set(x, 2, column.z);
    self.set(x, 3, column.w);
  }

  pub fn get_col(&self, x: usize) -> Vec4 {
    Vec4::new(
      self.get(x, 0),
      self.get(x, 1),
      self.get(x, 2),
      self.get(x, 3),
    )
  }

  pub fn get_algebraic_cofactor(&self, x: usize, y: usize) -> Mat3 {
    let mut result = Mat3::identity();
    for x_iter in 0..4 {
      if x_iter == x {
        continue;
      }
      for y_iter in 0..4 {
        if y_iter == y {
          continue;
        }

        let real_x = if x_iter > x { x_iter - 1 } else { x_iter };
        let real_y = if y_iter > y { y_iter - 1 } else { y_iter };
        result.set(real_x, real_y, self.get(x_iter, y_iter));
      }
    }
    result
  }

  pub fn get_cofactor(&self, x: usize, y: usize) -> Mat3 {
    self.get_algebraic_cofactor(x, y) * if (x + y) % 2 == 0 { 1 } else { -1 } as f32
  }

  pub fn det(&self) -> f32 {
    self.get_cofactor(0, 0).det() * self.get(0, 0)
      + self.get_cofactor(1, 0).det() * self.get(1, 0)
      + self.get_cofactor(2, 0).det() * self.get(2, 0)
      + self.get_cofactor(3, 0).det() * self.get(3, 0)
  }

  pub fn inverse_transpose(&self) -> Option<Mat4> {
    let d: f32 = self.det();
    if d.abs() <= std::f32::EPSILON {
      return None;
    }

    let mut result = Mat4::identity();
    for x in 0..4 {
      for y in 0..4 {
        result.set(x, y, self.get_cofactor(x, y).det() / d);
      }
    }
    Some(result)
  }

  pub fn inverse(&self) -> Option<Self> {
    self.inverse_transpose().map(|v| v.transpose())
  }
}

#[rustfmt::skip]
pub fn apply_translate(offset: &Vec3) -> Mat4 {
  Mat4::from_row(&[
    1.0, 0.0, 0.0, offset.x,
    0.0, 1.0, 0.0, offset.y,
    0.0, 0.0, 1.0, offset.z,
    0.0, 0.0, 0.0, 1.0,
  ])
}

#[rustfmt::skip]
pub fn apply_scale(scale: &Vec3) -> Mat4 {
  let Vec3{x,y,z} = scale;
  Mat4::from_row(&[
    *x  , 0.0, 0.0, 0.0,
    0.0, *y  , 0.0, 0.0,
    0.0, 0.0, *z  , 0.0,
    0.0, 0.0, 0.0, 1.0,
  ])
}

pub fn extract_position(mat: Mat4) -> Vec3 {
  Vec3::new(mat.get(0, 3), mat.get(1, 3), mat.get(2, 3))
}

pub fn extract_scale(mat: Mat4) -> Vec3 {
  let mut sx = Vec3::new(mat.get(0, 0), mat.get(0, 1), mat.get(0, 2)).length();
  let sy = Vec3::new(mat.get(1, 0), mat.get(1, 1), mat.get(1, 2)).length();
  let sz = Vec3::new(mat.get(2, 0), mat.get(2, 1), mat.get(2, 2)).length();

  if mat.det() < 0.0 {
    sx = -sx;
  }

  Vec3::new(sx, sy, sz)
}

pub fn decompose(mat: Mat4) -> (Vec3, Mat4, Vec3) {
  let scale = extract_scale(mat);
  let position = extract_position(mat);
  let mut rotate_matrix = Mat4::zeros();
  let scales = [scale.x, scale.y, scale.z];
  for i in 0..2 {
    rotate_matrix.set_col(i as usize, mat.get_col(i as usize) / scales[i]);
  }

  (position, rotate_matrix, scale)
}

use super::{Mat4, Vec4};

pub struct Euler {
  /// angle in degrees
  pub(super) x: f32,
  pub(super) y: f32,
  pub(super) z: f32,
}

impl Euler {
  pub fn make_rotate_matrix(&self) -> Mat4 {
    let Self { x, y, z } = self;
    Self::apply_eular_rotate_xyz(x.to_radians(), y.to_radians(), z.to_radians())
  }

  #[rustfmt::skip]
  /// angle in radian
  pub fn apply_eular_rotate_y(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from_row(&[
      c, 0.0,   s, 0.0,
      0.0, 1.0, 0.0, 0.0,
      -s, 0.0,   c, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  #[rustfmt::skip]
  /// angle in radian
  pub fn apply_eular_rotate_x(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from_row(&[
      1.0, 0.0, 0.0, 0.0,
      0.0,   c,  -s, 0.0,
      0.0,   s,   c, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  #[rustfmt::skip]
  /// angle in radian
  pub fn apply_eular_rotate_z(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from_row(&[
        c,  -s, 0.0, 0.0,
        s,   c, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// angle in radian
  pub fn apply_eular_rotate_xyz(x: f32, y: f32, z: f32) -> Mat4 {
    Self::apply_eular_rotate_z(z) * Self::apply_eular_rotate_y(y) * Self::apply_eular_rotate_x(x)
  }
}

impl From<Mat4> for Euler {
  fn from(rotate_matrix: Mat4) -> Self {
    let Vec4 {
      x: m11,
      y: m21,
      z: m31,
      ..
    } = rotate_matrix.get_col(0);
    let Vec4 {
      x: m12,
      y: m22,
      z: m32,
      ..
    } = rotate_matrix.get_col(1);
    let Vec4 {
      x: m13,
      y: m23,
      z: m33,
      ..
    } = rotate_matrix.get_col(2);

    let y = m13.clamp(-1.0, 1.0).asin();

    let (x, z) = if m13.abs() < 0.999999 {
      ((-m23).atan2(m33), (-m12).atan2(m11))
    } else {
      (m32.atan2(m22), 0.0)
    };

    Self { x, y, z }
  }
}

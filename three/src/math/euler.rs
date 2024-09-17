use super::Mat4;

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

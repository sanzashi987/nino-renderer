use super::{euler::Euler, quaternion::Quaternion, Mat4, Vec4};

pub struct Rotation {
  pub quaternion: Quaternion,
  pub euler: Euler,
}

impl Rotation {
  pub fn update_quaternion_from_matrix(&mut self, mat: Mat4) {
    self.quaternion.update_from_rotate_matrix(mat);
    self.update_to_euler();
  }

  pub fn set_quaternion(&mut self, q: Quaternion) {
    self.quaternion = q;
    self.update_to_euler();
  }

  pub fn set_euler(&mut self, e: Euler) {}

  fn update_to_euler(&mut self) {
    let rotate_matrix = self.quaternion.make_rotate_matrix();

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

    self.euler.y = m13.clamp(-1.0, 1.0).asin();

    (self.euler.x, self.euler.z) = if m13.abs() < 0.999999 {
      ((-m23).atan2(m33), (-m12).atan2(m11))
    } else {
      (m32.atan2(m22), 0.0)
    };
  }

  fn update_to_quaternion(&mut self) {
    let Euler { x, y, z } = self.euler;
    let (x, y, z) = (x.to_radians(), y.to_radians(), z.to_radians());

    let c1 = (x / 2.0).cos();
    let c2 = (y / 2.0).cos();
    let c3 = (z / 2.0).cos();

    let s1 = (x / 2.0).sin();
    let s2 = (y / 2.0).sin();
    let s3 = (z / 2.0).sin();

    // default order: XYZ
    self.quaternion.x = s1 * c2 * c3 + c1 * s2 * s3;
    self.quaternion.y = c1 * s2 * c3 - s1 * c2 * s3;
    self.quaternion.z = c1 * c2 * s3 + s1 * s2 * c3;
    self.quaternion.w = c1 * c2 * c3 - s1 * s2 * s3;
  }
}

use super::{euler::Euler, quaternion::Quaternion, Mat4, Vec3};
#[derive(Debug, Default)]
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

  pub fn set_euler(&mut self, e: Euler) {
    self.euler = e;
    self.update_to_quaternion();
  }

  pub fn quaternion_rotate(&mut self, axis: Vec3, angle: f32, premultiply: bool) {
    let q = Quaternion::from_axis_angle(axis, angle);
    self.quaternion = if premultiply {
      q * self.quaternion
    } else {
      self.quaternion * q
    };
    self.update_to_euler();
  }

  fn update_to_euler(&mut self) {
    let rotate_matrix = self.quaternion.make_rotate_matrix();
    let euler: Euler = rotate_matrix.into();
    self.euler = euler
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

impl From<Mat4> for Rotation {
  fn from(value: Mat4) -> Self {
    let euler: Euler = value.into();
    let mut res = Self {
      euler,
      quaternion: Default::default(),
    };

    res.update_to_quaternion();
    res
  }
}

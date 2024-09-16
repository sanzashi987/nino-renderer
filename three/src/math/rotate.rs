use super::{euler::Euler, quaternion::Quaternion, Mat4};

pub struct Rotation {
  pub quaternion: Quaternion,
  pub euler: Euler,
}

impl Rotation {
  pub fn update_quaternion_from_matrix(&mut self, mat: Mat4) {
    self.quaternion.update_from_rotate_matrix(mat);
  }

  pub fn set_quaternion(&mut self, q: Quaternion) {}

  pub fn set_euler(&mut self, e: Euler) {}
}

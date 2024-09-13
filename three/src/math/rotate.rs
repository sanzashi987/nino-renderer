use super::{euler::Euler, quaternion::Quaternion};

pub struct Rotation {
  pub quaternion: Quaternion,
  pub euler: Euler,
}

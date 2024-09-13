use crate::math::Mat4;

pub trait Camera {
  fn view_matrix(&self) -> Mat4;
  fn projection_matrix(&self) -> Mat4;
}

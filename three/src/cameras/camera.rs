use crate::{core::object_3d::IObject3D, math::Mat4};

pub trait ICamera: IObject3D {
  fn view_matrix(&self) -> Mat4;

  fn update_projection_matrix(&self);

  fn projection_matrix(&self) -> Mat4;

  fn project_matrix_inverse(&self) -> Mat4 {
    self.projection_matrix().inverse().unwrap()
  }

  fn global_matrix_inverse(&self) -> Mat4;
}

use crate::{core::object_3d::ObjectActions, math::Mat4};

pub trait ICamera: ObjectActions {
  fn view_matrix(&self) -> Mat4;
  fn projection_matrix(&self) -> Mat4;

  fn project_matrix_inverse(&self) -> Mat4 {
    self.projection_matrix().inverse().unwrap()
  }

  fn global_matrix_inverse(&self) -> Mat4 {
    self.update_global_matrix();
    self.global_matrix().inverse().unwrap()
  }
}

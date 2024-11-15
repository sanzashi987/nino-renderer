use std::{cell::RefCell, rc::Rc};

use renderer_macro_derive::object_3d;

use crate::{
  core::object_3d::{with_default_fields, IObject3D},
  math::Mat4,
};

use super::camera::ICamera;

#[object_3d(IObject3D)]
pub struct PerspectiveCamera {
  // in degree, stands for angle from top to bottom
  pub fov: f32,
  pub aspect: f32,
  pub near: f32,
  pub far: f32,
  pub focus: f32,
  pub zoom: f32,

  pub view_matrix: RefCell<Mat4>,
  pub projection_matrix: RefCell<Mat4>,
}

impl PerspectiveCamera {
  pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Rc<Self> {
    let (focus, zoom) = (10.0, 1.0);
    let (view_matrix, projection_matrix) =
      (RefCell::new(Mat4::zeros()), RefCell::new(Mat4::zeros()));

    let instance =
      with_default_fields!(Camera;fov,aspect,near,far,focus,zoom,view_matrix, projection_matrix);

    instance.update_perspective_mat();
    let that = instance.clone();
    let mut event_emitter = that.event_emitter.borrow_mut();
    let that_2 = instance.clone();
    event_emitter.on(
      "update:global_matrix",
      Box::new(move |x| {
        if let Ok(global_matrix) = x.downcast::<Mat4>() {
          let mut mutator = that_2.view_matrix.borrow_mut();
          *mutator = global_matrix.inverse().unwrap();
        }
      }),
    );

    instance
  }

  #[rustfmt::skip]
  pub fn update_perspective_mat(&self) {
    let top = self.near * (self.fov.to_radians() / 2.0).tan() / self.zoom;
    let height = top * 2.0;
    let width = self.aspect * height;
    let left = -0.5 * width;
    let right = left + width;
    let bottom = top - height;

    let x = 2.0 * self.near / (right - left);
    let y = 2.0 * self.near / (top - bottom);

    let a = (right + left) / (right - left);
    let b = (top + bottom) / (top - bottom);

    let base = self.near - self.far;
    let c = (self.far + self.near) / base;
    let d = 2.0 * self.far * self.near / base;

    let projection_matrix = Mat4::from_row(&[
        x,  0.0,    a,  0.0,
      0.0,    y,    b,  0.0,
      0.0,  0.0,    c,    d,
      0.0,  0.0, -1.0,  0.0,
    ]);

    let mut mutator = self.projection_matrix.borrow_mut();

    *mutator = projection_matrix;

  }
}

impl ICamera for PerspectiveCamera {
  fn view_matrix(&self) -> crate::math::Mat4 {
    self.global_matrix_inverse()
  }

  fn projection_matrix(&self) -> crate::math::Mat4 {
    *self.projection_matrix.borrow()
  }

  fn global_matrix_inverse(&self) -> Mat4 {
    *self.view_matrix.borrow()
  }

  fn update_projection_matrix(&self) {
    self.update_perspective_mat();
  }
}

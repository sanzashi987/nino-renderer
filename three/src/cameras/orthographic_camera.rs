use std::{cell::RefCell, rc::Rc};

use renderer_macro_derive::object_3d;

use crate::{
  core::object_3d::{with_default_fields, IObject3D},
  math::Mat4,
};

use super::camera::{derive_view_matrix, ICamera, View};

#[object_3d(IObject3D)]
pub struct OrthographicCamera {
  zoom: f32,
  view: Option<View>,
  left: f32,
  right: f32,
  top: f32,
  bottom: f32,
  near: f32,
  far: f32,

  view_matrix: RefCell<Mat4>,
  projection_matrix: RefCell<Mat4>,
  projection_matrix_inverse: RefCell<Mat4>,
}

impl ICamera for OrthographicCamera {
  fn view_matrix(&self) -> crate::math::Mat4 {
    *self.view_matrix.borrow()
  }

  fn update_projection_matrix(&self) {
    let dx = (self.right - self.left) / (2.0 * self.zoom);
    let dy = (self.top - self.bottom) / (2.0 * self.zoom);
    let cx = (self.right + self.left) / 2.0;
    let cy = (self.top + self.bottom) / 2.0;

    let mut left = cx - dx;
    let mut right = cx + dx;
    let mut top = cy + dy;
    let mut bottom = cy - dy;

    if let Some(v) = &self.view {
      if v.enabled {
        let scale_w = (self.right - self.left) / v.full_width / self.zoom;
        let scale_h = (self.top - self.bottom) / v.full_height / self.zoom;

        left += scale_w * v.offset_x;
        right = left + scale_w * v.width;
        top -= scale_h * v.offset_y;
        bottom = top - scale_h * v.height;
      }
    }

    let mat = orthographic_matrix(left, right, top, bottom, self.near, self.far);
    let mut p = self.projection_matrix.borrow_mut();
    *p = mat;
    let mut p_inv = self.projection_matrix_inverse.borrow_mut();
    *p_inv = mat.inverse().unwrap();
  }

  fn projection_matrix(&self) -> crate::math::Mat4 {
    *self.projection_matrix.borrow()
  }

  fn project_matrix_inverse(&self) -> crate::math::Mat4 {
    *self.projection_matrix_inverse.borrow()
  }
}

#[rustfmt::skip]
fn orthographic_matrix(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
  let w = 1.0 / (right - left);
  let h = 1.0 / (top - bottom);
  let p = 1.0 / (far - near);

  let x = (right + left) * w;
  let y = (top + bottom) * h;

  let z = (far + near) * p;
  let z_inv = -2.0 * p;

  Mat4::from_row([
    2.0 * w,    0.0,		  0.0, 		  -x,
		0.0, 		    2.0 * h,  0.0, 		  -y,
		0.0, 		    0.0,		  z_inv,	  -z,
		0.0, 		    0.0,		  0.0,		  1.0,
  ])
}

impl OrthographicCamera {
  pub fn new(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Rc<Self> {
    let zoom = 1.0;
    let view = None;
    let (view_matrix, projection_matrix, projection_matrix_inverse) = (
      RefCell::new(Mat4::zeros()),
      RefCell::new(Mat4::zeros()),
      RefCell::new(Mat4::zeros()),
    );

    let instance = with_default_fields!(Camera;zoom,view,left,right,top,bottom,near,far,view_matrix, projection_matrix,projection_matrix_inverse);

    derive_view_matrix!(instance);

    instance
  }

  pub fn default() -> Rc<Self> {
    Self::new(-1.0, 1.0, 1.0, -1.0, 0.1, 2000.0)
  }
}

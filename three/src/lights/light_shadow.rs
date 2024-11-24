use std::{borrow::BorrowMut, rc::Rc};

use crate::{
  cameras::camera::ICamera,
  core::{object_3d::IObject3D, render_target::RenderTarget},
  math::{Mat4, Vec2, Vec4},
};

use super::light::{ILight, ILightShadow};

#[rustfmt::skip]
static NDC_FACTOR: Mat4 = Mat4::from_row([			
  0.5, 0.0, 0.0, 0.5,
	0.0, 0.5, 0.0, 0.5,
	0.0, 0.0, 0.5, 0.5,
	0.0, 0.0, 0.0, 1.0
]);
pub struct LightShadow {
  camera: Rc<dyn ICamera>,
  intensity: i32,
  bias: i32,
  normal_bias: i32,
  radius: i32,
  // shadow texture width & height
  map_size: Vec2,
  mat: Mat4,
  // vec4 -> offsetx, offsety, width, height
  viewports: Vec<Vec4>,
  map: Option<RenderTarget>,

  // the standard `NDC` VP matrix from the shadow
  // the target & position will automatically align with the light
  matrix: std::cell::RefCell<Mat4>,
}

impl ILightShadow for LightShadow {
  fn matrix(&self) -> Mat4 {
    *self.matrix.borrow()
  }

  fn camera(&self) -> Rc<dyn ICamera> {
    self.camera.clone()
  }

  fn map_size(&self) -> Vec2 {
    self.map_size
  }

  fn viewports(&self) -> &Vec<Vec4> {
    &self.viewports
  }

  fn update_matrices(&self, light: Rc<dyn ILight>, viewport: Vec4) {
    let global_light_position = light.global_matrix().get_col(3).truncated_to_vec3();
    self
      .camera
      .update_from_global_position(global_light_position);
    let target_position = light
      .target()
      .global_matrix()
      .get_col(3)
      .truncated_to_vec3();

    self.camera.look_at(target_position);
    self.camera.update_global_matrix();

    let vp_matrix = self.camera.projection_matrix() * self.camera.global_matrix_inverse();

    let mut matrix = self.matrix.borrow_mut();
    *matrix = NDC_FACTOR * vp_matrix;
  }

  fn map(&self) -> &RenderTarget {
    todo!()
  }
}

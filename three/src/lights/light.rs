use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  core::{object_3d::IObject3D, uniform::Uniform},
  math::{Mat4, Vec2, Vec3, Vec4},
  core::render_target::RenderTarget,
};

pub enum LightType {
  AmbientLight,
  DirectionalLight,
  LightProbe,
  PointLight,
  SpotLight,
  HemisphereLight,
  RectAreaLight,
}

pub trait LightToUniform {
  fn to_uniform(&self, camera: Rc<dyn ICamera>) -> Uniform;
}

pub trait ILight: IObject3D + LightToUniform {
  fn light_type(&self) -> LightType;

  fn to_shadow_uniform(&self) -> Uniform;

  fn shadow(&self) -> Option<Rc<dyn ILightShadow>>;
}

pub trait ILightShadow {
  fn map_size(&self) -> Vec2;

  fn viewports(&self) -> &Vec<Vec4>;

  fn update_matrices(&self);

  fn map(&self) -> &RenderTarget;
}

pub fn compute_direction(position: Vec3, target: Vec3, view_matrix: Mat4) -> Vec3 {
  let Vec3 { x, y, z } = position - target;
  let mut direction = Vec3::zero();
  direction.x = view_matrix.get(0, 0) * x + view_matrix.get(1, 0) * y + view_matrix.get(2, 0) * z;
  direction.y = view_matrix.get(0, 1) * x + view_matrix.get(1, 1) * y + view_matrix.get(2, 1) * z;
  direction.z = view_matrix.get(0, 2) * x + view_matrix.get(1, 2) * y + view_matrix.get(2, 2) * z;
  direction
}

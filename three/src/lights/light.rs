use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  core::{object_3d::IObject3D, uniform::Uniform},
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

pub trait ILightShadow {}

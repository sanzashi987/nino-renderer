use crate::core::{object_3d::ObjectActions, uniform::Uniform};

pub trait ILight: ObjectActions {
  fn to_uniform(&self) -> Uniform;

  fn to_shadow_uniform(&self) -> Uniform;

  fn shadow(&self) -> dyn ILightShadow;
}

pub trait ILightShadow {}

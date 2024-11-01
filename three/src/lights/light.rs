use crate::core::{object_3d::IObject3D, uniform::Uniform};

pub trait ILight: IObject3D {
  fn to_uniform(&self) -> Uniform;

  fn to_shadow_uniform(&self) -> Uniform;

  fn shadow(&self) -> dyn ILightShadow;
}

pub trait ILightShadow {}

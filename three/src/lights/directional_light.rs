use std::rc::Rc;

use renderer_macro_derive::object_3d;

use crate::{
  core::object_3d::{with_default_fields, IObject3D},
  math::Vec4,
  objects::base::Object3D,
};

use super::light::ILight;

#[object_3d(IObject3D)]
pub struct DirectionalLight {
  pub color: Vec4,
  pub intensity: f32,
  pub target: Object3D,
  // pub shadow
}

impl DirectionalLight {
  pub fn new() -> std::rc::Rc<Self> {
    let color = Vec4::default();
    let intensity = 1.0f32;
    let target = Object3D::new_ownership();
    let this = with_default_fields!(Light;color,intensity,target);
    this
  }
}

impl ILight for DirectionalLight {
  fn to_uniform(&self) -> crate::core::uniform::Uniform {
    todo!()
  }

  fn to_shadow_uniform(&self) -> crate::core::uniform::Uniform {
    todo!()
  }

  fn shadow(&self) -> Rc<dyn super::light::ILightShadow> {
    todo!()
  }
}

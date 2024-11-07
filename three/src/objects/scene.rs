use renderer_macro_derive::object_3d;

use crate::math::Vec4;

use super::{
  super::core::object_3d::{with_default_fields, IObject3D},
  // group::GroupSupportChildren,
};

pub struct Fog {
  near: f32,
  far: f32,
  color: Vec4,
}

impl Default for Fog {
  fn default() -> Self {
    Self {
      near: Default::default(),
      far: Default::default(),
      color: Default::default(),
    }
  }
}

#[object_3d(IObject3D)]
pub struct Scene {
  fog: Fog,
}

impl Scene {
  pub fn new() -> std::rc::Rc<Self> {
    let fog = Default::default();
    let this = with_default_fields!(Scene;fog);
    this
  }
}

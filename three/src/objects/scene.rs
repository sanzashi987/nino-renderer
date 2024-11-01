use renderer_macro_derive::object_3d;

use super::{
  super::core::object_3d::{with_default_fields, IObject3D},
  // group::GroupSupportChildren,
};

#[object_3d(IObject3D)]
pub struct Scene {}

impl Scene {
  pub fn new() -> std::rc::Rc<Self> {
    let this = with_default_fields!(Scene);
    this
  }
}

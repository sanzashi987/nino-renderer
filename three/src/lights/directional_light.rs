use renderer_macro_derive::object_3d;

use crate::{
  core::object_3d::{with_default_fields, ObjectActions},
  math::Vec4,
  objects::base::Object3D,
};

#[object_3d(ObjectActions)]
pub struct DirectionalLight {
  pub color: Vec4,
  pub intensity: f32,
  pub target: Object3D,
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

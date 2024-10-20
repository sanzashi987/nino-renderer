use renderer_macro_derive::object_3d;

use crate::{
  core::object_3d::{with_default_fields, ObjectActions},
  math::Vec4,
};

#[object_3d(ObjectActions)]
pub struct Light {
  pub color: Vec4,
  pub intensity: f32,
}

impl Light {
  pub fn new() -> std::rc::Rc<Self> {
    let color = Vec4::default();
    let intensity = 1.0f32;
    let this = with_default_fields!(Light;color,intensity);
    this
  }
}

use std::collections::HashMap;

use crate::math::{Mat4, Vec2, Vec3, Vec4};

use super::marco::define_gl_type_enum;

pub trait Extract<T> {
  fn extract(self) -> Option<T>;
}

define_gl_type_enum!(
  UnifromTypeEnum;
  Int-i32,
  Float-f32,
  Vec2-Vec2,
  Vec3-Vec3,
  Vec4-Vec4,
  Mat4-Mat4
);

pub struct Uniform {
  attributes: HashMap<String, UnifromTypeEnum>,
}

trait SetGlType<T> {
  fn set_attribute(&mut self, key: String, val: T);
}

impl Uniform {
  pub fn remove_attribute(&mut self, key: String) {
    self.attributes.remove(&key);
  }
}

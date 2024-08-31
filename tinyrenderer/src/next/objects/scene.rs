use std::any::{type_name, type_name_of_val};

use crate::{math::Vec2, next::core::object_3d::ObjectType};

use super::super::core::object_3d::Object3D;

trait C<S> {
  fn convert(self) -> Option<S> {
    None
  }
}

enum SceneSupportChild {
  s,
}

impl SceneSupportChild {
  const supportedValue: &'static [&'static str] = &[type_name::<Vec2>()];

  pub fn convert<T>(val: T) -> Option<Self> {
    let input_type_name = type_name::<T>();
    let vec = type_name::<Vec2>();

    

    let a = Self::supportedValue.contains(&input_type_name);
    None
  }
}

pub struct Scene {
  base: Object3D<SceneSupportChild>,
}
fn a() {
  let a = SceneSupportChild::s;
  type_name_of_val(&a)
}

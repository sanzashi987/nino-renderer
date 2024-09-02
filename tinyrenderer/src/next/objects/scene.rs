use std::any::{type_name, type_name_of_val, Any};

use crate::{
  math::Vec2,
  next::core::object_3d::{define_support_objects, Object3DMethod},
};

use super::{super::core::object_3d::Object3D, group::GroupSupportChildren};

pub struct Scene {
  base: Object3D<GroupSupportChildren>,
}

impl Object3DMethod for Scene {
  fn add<T: 'static + Sized>(&mut self, object: T) -> bool {
    if let Some(e) = GroupSupportChildren::convert(object) {
      self.base.add(e);
      return true;
    }
    return false;
  }
}

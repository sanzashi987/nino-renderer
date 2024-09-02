use std::any::{type_name, type_name_of_val, Any};

use crate::{
  math::Vec2,
  next::core::object_3d::{define_support_objects, Object3DMethod, ObjectType},
};

use super::{super::core::object_3d::Object3D, group::Group, mesh::Mesh};

define_support_objects!(
  SceneSupportChildren;
  Group:Group,
  Mesh:Mesh
);

pub struct Scene {
  base: Object3D<SceneSupportChildren>,
}

impl Object3DMethod for Scene {
  fn add<T: 'static + Sized>(&mut self, object: T) -> bool {
    if let Some(e) = SceneSupportChildren::convert(object) {
      self.base.add(e);
      return true;
    }
    return false;
  }
}

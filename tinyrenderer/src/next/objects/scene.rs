use std::any::{type_name, type_name_of_val, Any};

use crate::{
  math::Vec2,
  next::core::object_3d::{define_support_objects, ObjectType},
};

use super::{super::core::object_3d::Object3D, group::Group};

define_support_objects!(
  SceneSupportChildren;
  Group:Group
);

pub struct Scene {
  base: Object3D<SceneSupportChildren>,
}

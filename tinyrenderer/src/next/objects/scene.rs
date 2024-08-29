use crate::next::core::object_3d::ObjectType;

use super::super::core::object_3d::Object3D;

enum SceneSupportChild {}

pub struct Scene {
  base: Object3D<SceneSupportChild>,
}

use renderer_macro_derive::object_3d;

use super::{
  super::core::object_3d::{with_default_fields, ObjectActions},
  // group::GroupSupportChildren,
};

#[object_3d(ObjectActions)]
pub struct Scene {}

// impl Scene {
//   pub fn new() -> Self {
//     with_default_fields![]
//   }
// }

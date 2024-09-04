use std::ops::{Deref, DerefMut};

use crate::next::core::object_3d::Transform;

use super::super::{
  core::object_3d::{define_support_objects, Object3D, Object3DMethod},
  lights::light::Light,
};

use super::mesh::Mesh;

define_support_objects!(
  GroupSupportChildren;
  Group:Group,
  Mesh:Mesh,
  Light:Light
);

// pub enum GroupSupportChildren {
//   Group(Group),
// }
// impl GroupSupportChildren {
//   pub fn convert<T: 'static + Sized>(val: T) -> Option<Self> {
//     let val_any: Box<dyn std::any::Any> = Box::new(val);
//     match val_any.downcast::<Group>() {
//       Ok(matched) => {
//         return Some(Self::Group(*matched));
//       }
//       _ => {}
//     }
//     return None;
//   }
// }
pub struct Group {
  base: Object3D<GroupSupportChildren>,
}

impl Group {
  pub fn new() -> Self {
    Group {
      base: Object3D::<GroupSupportChildren>::new(),
    }
  }
}

impl Object3DMethod for Group {
  fn add<T: 'static + Sized>(&mut self, object: T) -> bool {
    if let Some(e) = GroupSupportChildren::convert(object) {
      self.base.add(e);
      return true;
    }
    return false;
  }
}

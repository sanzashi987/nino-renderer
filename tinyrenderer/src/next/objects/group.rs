use crate::next::core::object_3d::{define_support_objects, Object3D};

define_support_objects!(
  GroupSupportChildren;
  Group:Group
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
  // v: Vec<Group>,
}

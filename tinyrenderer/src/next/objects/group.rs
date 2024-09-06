use super::super::{
  core::object_3d::{define_support_objects, with_default_fields, ObjectActions},
  lights::light::Light,
};

use renderer_macro_derive::object_3d;

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
#[object_3d(GroupSupportChildren, ObjectActions)]
pub struct Group {}

// impl ObjectActions for Group {
//   fn transform_matrix(&self) -> &crate::math::Mat4 {
//     &self.matrix
//   }
//   fn set_parent(&self, parent: std::rc::Rc<dyn ObjectActions>) {
//     let mut p = self.parent.borrow_mut();
//     *p = Some(parent);
//   }
//   fn get_parent(&self) -> Option<std::rc::Rc<dyn ObjectActions>> {
//     if let Some(p) = self.parent.borrow().as_ref() {
//       Some(p.clone())
//     } else {
//       None
//     }
//   }
// }

impl Group {
  pub fn new() -> Self {
    with_default_fields![]
  }
}

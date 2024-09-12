use std::{borrow::Borrow, ops::Deref};

use crate::math::{Mat4, Vec3};

use super::super::{
  core::object_3d::{define_support_objects, with_default_fields, ObjectActions},
  // lights::light::Light,
};

use renderer_macro_derive::object_3d;

// use super::mesh::Mesh;

// define_support_objects!(
//   GroupSupportChildren;
//   Group:Group,
//   Mesh:Mesh,
//   Light:Light
// );

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
// #[object_3d(ObjectActions)]
// pub struct Group {}

pub struct Group {
  parent: std::cell::RefCell<Option<std::rc::Rc<dyn ObjectActions>>>,
  children: std::cell::RefCell<Vec<Box<dyn ObjectActions>>>,
  matrix: std::cell::RefCell<crate::math::Mat4>,
  global_matrix: std::cell::RefCell<crate::math::Mat4>,
  position: crate::math::Vec3,
  rotation: crate::math::Vec3,
  scale: crate::math::Vec3,
  visible: bool,
  cast_shadow: bool,
  receive_shadow: bool,
  user_data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
  is_camera: bool,
  is_light: bool,
  // matrix_world_auto_update: bool,
}
impl ObjectActions for Group {
  fn matrix(&self) -> crate::math::Mat4 {
    *self.matrix.borrow()
  }
  fn global_matrix(&self) -> crate::math::Mat4 {
    *self.global_matrix.borrow()
  }
  fn set_parent(&self, parent: std::rc::Rc<dyn ObjectActions>) {
    let mut p = self.parent.borrow_mut();
    *p = Some(parent);
  }
  fn get_parent(&self) -> Option<std::rc::Rc<dyn ObjectActions>> {
    if let Some(p) = self.parent.borrow().as_ref() {
      Some(p.clone())
    } else {
      None
    }
  }
  fn add(&self, val: Box<dyn ObjectActions>) {
    let mut children = self.children.borrow_mut();
    children.push(val);
  }

  fn look_at(&self, target: crate::math::Vec3) {
    // let back = (self.position - target).normalize();
    self.update_global_matrix();

    let position = self.global_matrix.borrow().extract_position();

    let (eye, target) = if self.is_camera || self.is_light {
      // camera default looking back along -z;
      (position, target)
    } else {
      (target, position)
    };

    let orthogonal_basis = crate::math::Mat3::get_orthogonal_basis(eye, target, *Vec3::y_axis());

    let looking_at = target - self.position;

    // self.view_direction = back * -1.0;
  }

  fn update_global_matrix(&self) {
    self.update_matrix();

    if let Some(parent) = self.parent.borrow().as_ref() {
      parent.update_global_matrix();
      let parent_global = parent.global_matrix();
      let mut global_matrix = self.global_matrix.borrow_mut();
      *global_matrix = parent_global * *global_matrix;
    }

    for child in self.children.borrow().deref() {
      child.update_global_matrix();
    }
  }

  fn update_matrix(&self) {
    let next_matrix = Mat4::compose(self.position, self.rotation, self.scale);
    let mut matrix = self.matrix.borrow_mut();
    *matrix = next_matrix;
  }
}

// impl Group {
//   pub fn new() -> Self {
//     with_default_fields![]
//   }
// }

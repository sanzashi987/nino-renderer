use super::super::core::object_3d::ObjectActions;

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
  position: std::cell::RefCell<crate::math::Vec3>,
  rotation: std::cell::RefCell<crate::math::rotate::Rotation>,
  scale: std::cell::RefCell<crate::math::Vec3>,
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

    let position = crate::math::extract_position(*self.global_matrix.borrow());

    let (eye, target) = if self.is_camera || self.is_light {
      // camera default looking back along -z;
      (position, target)
    } else {
      (target, position)
    };

    let orthogonal_basis =
      crate::math::Mat3::get_orthogonal_basis(eye, target, *crate::math::Vec3::y_axis());

    let looking_at = target - *self.position.borrow();

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

    for child in std::ops::Deref::deref(&self.children.borrow()) {
      child.update_global_matrix();
    }
  }

  fn update_matrix(&self) {
    let next_matrix = self.compose();
    let mut matrix = self.matrix.borrow_mut();
    *matrix = next_matrix;
  }

  fn compose(&self) -> crate::math::Mat4 {
    let translate_matrix = crate::math::apply_translate(&self.position.borrow());
    let rotate_matrix = (*self.rotation.borrow()).quaternion.make_rotate_matrix();
    let scale_matrix = crate::math::apply_scale(&self.scale.borrow());

    translate_matrix * rotate_matrix * scale_matrix
  }
  /// refer to http://facweb.cs.depaul.edu/andre/gam374/extractingTRS.pdf
  fn decompose(&self) {
    let mat = *self.matrix.borrow();
    let scale = crate::math::extract_scale(mat);
    let position = crate::math::extract_position(mat);

    let mut rotate_matrix = crate::math::Mat4::zeros();
    let scales = [scale.x, scale.y, scale.z];
    for i in 0..2 {
      rotate_matrix.set_col(i as usize, mat.get_col(i as usize) / scales[i]);
    }

    {
      self
        .rotation
        .borrow_mut()
        .update_quaternion_from_matrix(rotate_matrix);
    }

    {
      let mut scale_ref = self.scale.borrow_mut();
      *scale_ref = scale;
    }
    {
      let mut position_ref = self.position.borrow_mut();
      *position_ref = position;
    }
  }

  fn apply_matrix(&self, matrix: crate::math::Mat4) {
    self.update_matrix();
    let next_matrix = matrix * *self.matrix.borrow();
    let mut matrix_ref = self.matrix.borrow_mut();
    *matrix_ref = next_matrix;
    self.decompose();
  }

  fn apply_quaternion(&self, q: crate::math::Quaternion) {
    let next_q = (*self.rotation.borrow()).quaternion * q;
    let mut rotation_ref = self.rotation.borrow_mut();
    rotation_ref.set_quaternion(next_q);
  }

  fn attach(&self, child: Box<dyn ObjectActions>) {
    self.update_global_matrix();

    let global_matrix_invert = self
      .global_matrix
      .borrow()
      .inverse()
      .expect("expected a invertable global matrix");

    let mut res = global_matrix_invert;

    if let Some(parent) = child.get_parent() {
      parent.update_global_matrix();
      res = res * parent.global_matrix();
    }
  }
}

// impl Group {
//   pub fn new() -> Self {
//     with_default_fields![]
//   }
// }

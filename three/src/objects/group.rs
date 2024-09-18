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
  children: std::cell::RefCell<Vec<std::rc::Rc<dyn ObjectActions>>>,
  matrix: std::cell::RefCell<crate::math::Mat4>,
  global_matrix: std::cell::RefCell<crate::math::Mat4>,
  position: std::cell::RefCell<crate::math::Vec3>,
  rotation: std::cell::RefCell<crate::math::Rotation>,
  scale: std::cell::RefCell<crate::math::Vec3>,
  visible: bool,
  cast_shadow: bool,
  receive_shadow: bool,
  user_data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
  is_camera: bool,
  is_light: bool,
  _self_ref: std::rc::Weak<dyn ObjectActions>,
  _uuid: String,
  // matrix_world_auto_update: bool,
}

impl ObjectActions for Group {
  fn matrix(&self) -> crate::math::Mat4 {
    *self.matrix.borrow()
  }
  fn global_matrix(&self) -> crate::math::Mat4 {
    *self.global_matrix.borrow()
  }

  fn remove_parent(&self) {
    let p = self.parent.borrow_mut();

    if let Some(parent) = *p {}
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

  fn add(&self, child: std::rc::Rc<dyn ObjectActions>) {
    let mut children = self.children.borrow_mut();

    if let Some(me) = self._self_ref.upgrade() {
      child.remove_parent();
      child.set_parent(me.clone());
      children.push(child.clone());
    }
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

    let mut rotate_mat = crate::math::Mat4::identity();

    for i in 0..2 {
      let col = crate::math::Vec4::from_vec3(&orthogonal_basis.get_col(i), 0.0);
      rotate_mat.set_col(i, col);
    }

    let mut q: crate::math::Quaternion = rotate_mat.into();

    if let Some(parent) = *self.parent.borrow() {
      let (_, r, _) = crate::math::decompose(parent.global_matrix());

      let q_parent: crate::math::Quaternion = r.into();

      q = q_parent.inverse() * q;
    }

    {
      let mut rotate_ref = self.rotation.borrow_mut();
      rotate_ref.set_quaternion(q);
    }
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
    let (position, rotate_matrix, scale) = crate::math::decompose(mat);

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

    let mut res = self
      .global_matrix
      .borrow()
      .inverse()
      .expect("expected a invertable global matrix");

    if let Some(parent) = child.get_parent() {
      parent.update_global_matrix();
      res = res * parent.global_matrix();
    }

    child.apply_matrix(res);
  }

  fn remove(&self) {}

  fn global_scale(&self) -> crate::math::Vec3 {
    self.update_global_matrix();
    let mat = self.global_matrix();
    let (_, _, scale) = crate::math::decompose(mat);
    scale
  }

  fn global_position(&self) -> crate::math::Vec3 {
    self.update_global_matrix();
    let mat = self.global_matrix();
    let (position, _, _) = crate::math::decompose(mat);
    position
  }

  fn global_rotation(&self) -> crate::math::Rotation {
    self.update_global_matrix();
    let mat = self.global_matrix();
    let (_, rotation, _) = crate::math::decompose(mat);

    rotation.into()
  }
}

// impl Group {
//   pub fn new() -> Self {
//     with_default_fields![]
//   }
// }

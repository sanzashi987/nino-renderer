use crate::core::object_3d::{with_default_fields, ObjectActions};

use renderer_macro_derive::object_3d;

#[object_3d(ObjectActions)]
pub struct Group {}

impl Group {
  pub fn new() -> std::rc::Rc<Self> {
    let this = with_default_fields!(Group);
    this
  }
}

// pub struct Group {
//   parent: std::cell::RefCell<Option<std::rc::Rc<dyn ObjectActions>>>,
//   children: std::cell::RefCell<Vec<std::rc::Rc<dyn ObjectActions>>>,
//   matrix: std::cell::RefCell<crate::math::Mat4>,
//   global_matrix: std::cell::RefCell<crate::math::Mat4>,
//   position: std::cell::RefCell<crate::math::Vec3>,
//   rotation: std::cell::RefCell<crate::math::Rotation>,
//   scale: std::cell::RefCell<crate::math::Vec3>,
//   layers: std::cell::RefCell<crate::core::layer::Layers>,
//   cast_shadow: bool,
//   receive_shadow: bool,
//   visible: std::cell::RefCell<bool>,
//   user_data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
//   object_type: crate::core::object_3d::ObjectType,
//   _self_ref: std::cell::OnceCell<std::rc::Weak<dyn ObjectActions>>,
//   _uuid: String,
//   // matrix_world_auto_update: bool,
// }

// impl ObjectActions for Group {
//   fn parent(&self) -> Option<std::rc::Rc<dyn ObjectActions>> {
//     if let Some(p) = self.parent.borrow().as_ref() {
//       Some(p.clone())
//     } else {
//       None
//     }
//   }

//   fn set_parent(&self, parent: std::rc::Rc<dyn ObjectActions>) {
//     let mut p = self.parent.borrow_mut();
//     *p = Some(parent);
//   }

//   fn remove_from_parent(&self) {
//     let mut p = self.parent.borrow_mut();

//     if let Some(parent) = p.as_ref() {
//       parent.remove(&self._uuid);
//     }

//     *p = None;
//   }

//   fn remove(&self, uuid: &str) {
//     let mut children = self.children.borrow_mut();

//     if let Some(index) = children.iter().position(|x| (*x).uuid() == uuid) {
//       children.remove(index);
//     }
//   }

//   fn add(&self, child: std::rc::Rc<dyn ObjectActions>) {
//     let mut children = self.children.borrow_mut();

//     if let Some(self_pointer) = self._self_ref.get() {
//       if let Some(me) = self_pointer.upgrade() {
//         child.remove_from_parent();
//         child.set_parent(me.clone());
//         children.push(child.clone());
//       }
//     }
//   }

//   fn clear(&self) {
//     {
//       let children = self.children.borrow();
//       for child in children.iter() {
//         child.remove_from_parent();
//       }
//     }
//     let mut children = self.children.borrow_mut();

//     *children = vec![];
//   }

//   fn attach(&self, child: Box<dyn ObjectActions>) {
//     self.update_global_matrix();

//     let mut res = self
//       .global_matrix
//       .borrow()
//       .inverse()
//       .expect("expected a invertable global matrix");

//     if let Some(parent) = child.parent() {
//       parent.update_global_matrix();
//       res = res * parent.global_matrix();
//     }

//     child.apply_matrix(res);
//   }

//   fn children(&self) -> std::cell::Ref<'_, Vec<std::rc::Rc<dyn ObjectActions>>> {
//     self.children.borrow()
//   }

//   fn look_at(&self, target: crate::math::Vec3) {
//     // let back = (self.position - target).normalize();
//     self.update_global_matrix();

//     let position = crate::math::extract_position(*self.global_matrix.borrow());

//     let is_revert_z = self.object_type == crate::core::object_3d::ObjectType::Camera
//       || self.object_type == crate::core::object_3d::ObjectType::Light;

//     let (eye, target) = if is_revert_z {
//       // camera default looking back along -z;
//       (position, target)
//     } else {
//       (target, position)
//     };

//     let orthogonal_basis =
//       crate::math::Mat3::get_orthogonal_basis(eye, target, *crate::math::Vec3::y_axis());

//     let mut rotate_mat = crate::math::Mat4::identity();

//     for i in 0..2 {
//       let col = crate::math::Vec4::from_vec3(&orthogonal_basis.get_col(i), 0.0);
//       rotate_mat.set_col(i, col);
//     }

//     let mut q: crate::math::Quaternion = rotate_mat.into();

//     if let Some(parent) = self.parent.borrow().as_ref() {
//       let (_, r, _) = crate::math::decompose(parent.global_matrix());

//       let q_parent: crate::math::Quaternion = r.into();

//       q = q_parent.inverse() * q;
//     }

//     {
//       let mut rotate_ref = self.rotation.borrow_mut();
//       rotate_ref.set_quaternion(q);
//     }
//   }

//   fn matrix(&self) -> crate::math::Mat4 {
//     *self.matrix.borrow()
//   }

//   fn global_matrix(&self) -> crate::math::Mat4 {
//     *self.global_matrix.borrow()
//   }

//   fn update_global_matrix(&self) {
//     self.update_matrix();

//     if let Some(parent) = self.parent.borrow().as_ref() {
//       parent.update_global_matrix();
//       let parent_global = parent.global_matrix();
//       let mut global_matrix = self.global_matrix.borrow_mut();
//       *global_matrix = parent_global * *global_matrix;
//     }

//     for child in std::ops::Deref::deref(&self.children.borrow()) {
//       child.update_global_matrix();
//     }
//   }

//   fn update_matrix(&self) {
//     let next_matrix = self.compose();
//     let mut matrix = self.matrix.borrow_mut();
//     *matrix = next_matrix;
//   }

//   fn compose(&self) -> crate::math::Mat4 {
//     let translate_matrix = crate::math::apply_translate(&self.position.borrow());
//     let rotate_matrix = (*self.rotation.borrow()).quaternion.make_rotate_matrix();
//     let scale_matrix = crate::math::apply_scale(&self.scale.borrow());

//     translate_matrix * rotate_matrix * scale_matrix
//   }
//   /// refer to http://facweb.cs.depaul.edu/andre/gam374/extractingTRS.pdf
//   fn decompose(&self) {
//     let mat = *self.matrix.borrow();
//     let (position, rotate_matrix, scale) = crate::math::decompose(mat);

//     {
//       self
//         .rotation
//         .borrow_mut()
//         .update_quaternion_from_matrix(rotate_matrix);
//     }

//     {
//       let mut scale_ref = self.scale.borrow_mut();
//       *scale_ref = scale;
//     }
//     {
//       let mut position_ref = self.position.borrow_mut();
//       *position_ref = position;
//     }
//   }

//   fn apply_matrix(&self, matrix: crate::math::Mat4) {
//     self.update_matrix();
//     let mut next_matrix = crate::math::Mat4::zeros();
//     {
//       next_matrix = matrix * *self.matrix.borrow();
//     }
//     {
//       let mut matrix_ref = self.matrix.borrow_mut();
//       *matrix_ref = next_matrix;
//     }
//     self.decompose();
//   }

//   fn apply_quaternion(&self, q: crate::math::Quaternion) {
//     let next_q = (*self.rotation.borrow()).quaternion * q;
//     let mut rotation_ref = self.rotation.borrow_mut();
//     rotation_ref.set_quaternion(next_q);
//   }

//   fn rotate_on_world_axis(&self, axis: crate::math::Vec3, angle: f32) {
//     let mut rotate = self.rotation.borrow_mut();
//     // apply the rotation first(before any rotation occurs)
//     // means the rotate axis is defined in the world coordinate
//     rotate.quaternion_rotate(axis, angle, true)
//   }

//   fn rotate_on_axis(&self, axis: crate::math::Vec3, angle: f32) {
//     let mut rotate = self.rotation.borrow_mut();
//     rotate.quaternion_rotate(axis, angle, false)
//   }

//   fn rotate_x(&self, angle: f32) {
//     self.rotate_on_axis(*crate::math::Vec3::x_axis(), angle);
//   }

//   fn rotate_y(&self, angle: f32) {
//     self.rotate_on_axis(*crate::math::Vec3::y_axis(), angle);
//   }

//   fn rotate_z(&self, angle: f32) {
//     self.rotate_on_axis(*crate::math::Vec3::z_axis(), angle);
//   }

//   fn translate_on_axis(&self, axis: crate::math::Vec3, distance: f32) {
//     let q = self.rotation.borrow().quaternion;
//     let crate::math::Vec3 {
//       x: vx,
//       y: vy,
//       z: vz,
//     } = axis;
//   }

//   fn translate_x(&self, distance: f32) {}
//   fn translate_y(&self, distance: f32) {}
//   fn translate_z(&self, distance: f32) {}

//   fn global_scale(&self) -> crate::math::Vec3 {
//     self.update_global_matrix();
//     let mat = self.global_matrix();
//     let (_, _, scale) = crate::math::decompose(mat);
//     scale
//   }

//   fn global_position(&self) -> crate::math::Vec3 {
//     self.update_global_matrix();
//     let mat = self.global_matrix();
//     let (position, _, _) = crate::math::decompose(mat);
//     position
//   }

//   fn global_rotation(&self) -> crate::math::Rotation {
//     self.update_global_matrix();
//     let mat = self.global_matrix();
//     let (_, rotation, _) = crate::math::decompose(mat);

//     rotation.into()
//   }

//   fn test_layers(&self, layers: &crate::core::layer::Layers) -> bool {
//     self.layers.borrow().test(layers)
//   }

//   fn layers(&self) -> std::cell::Ref<crate::core::layer::Layers> {
//     self.layers.borrow()
//   }

//   fn visible(&self) -> bool {
//     *self.visible.borrow()
//   }

//   fn get_type(&self) -> crate::core::object_3d::ObjectType {
//     self.object_type
//   }

//   fn uuid(&self) -> &str {
//     &self._uuid
//   }
// }

// impl Group {
//   pub fn new() -> std::rc::Rc<Self> {
//     let this = std::rc::Rc::new(Self {
//       parent: Default::default(),
//       children: Default::default(),
//       matrix: Default::default(),
//       global_matrix: Default::default(),
//       position: Default::default(),
//       rotation: Default::default(),
//       scale: Default::default(),
//       visible: Default::default(),
//       layers: Default::default(),
//       cast_shadow: Default::default(),
//       object_type: crate::core::object_3d::ObjectType::Group,
//       receive_shadow: Default::default(),
//       user_data: Default::default(),
//       _uuid: uuid::Uuid::new_v4().to_string(),
//       _self_ref: Default::default(),
//     });

//     let that: std::rc::Rc<dyn crate::core::object_3d::ObjectActions> = this.clone();

//     let _ = this._self_ref.set(std::rc::Rc::downgrade(&that));

//     this

//     // with_default_fields!(Group;)
//   }
// }

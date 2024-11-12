use crate::core::object_3d::{with_default_fields, IObject3D};

use renderer_macro_derive::object_3d;

#[object_3d(IObject3D)]
pub struct Group {
  pub group_order: i32,
}

impl Group {
  pub fn new() -> std::rc::Rc<Self> {
    let group_order = 0i32;
    let this = with_default_fields!(Group;group_order);
    this
  }
}

use std::{fmt::Debug, rc::Rc};

use renderer_macro_derive::object_3d;

use crate::{
  core::{
    buffer_geometry::IGeometry,
    object_3d::{with_default_fields, ObjectActions},
  },
  material::material::IMaterial,
};

pub trait Renderable {
  fn geometry(&self) -> Rc<dyn IGeometry>;

  fn material(&self) -> Rc<dyn IMaterial>;
}

#[object_3d(ObjectActions)]
pub struct Object3D {}

impl Object3D {
  pub fn new() -> Rc<Self> {
    with_default_fields!(Object3D)
  }

  pub fn new_ownership() -> Self {
    let rc = Self::new();

    Rc::try_unwrap(rc).unwrap()
  }
}

use std::rc::Rc;

use renderer_macro_derive::object_3d;

use crate::{
  core::{
    buffer_geometry::{BufferGeometry, IGeometry},
    object_3d::{with_default_fields, ObjectActions},
  },
  material::{material::IMaterial, standard_material::StandardMeshMaterial},
};

use super::base::Renderable;

#[object_3d(ObjectActions)]
pub struct Line {
  geometry: Rc<BufferGeometry>,
  material: Rc<StandardMeshMaterial>,
}

impl Line {
  pub fn new() -> std::rc::Rc<Self> {
    let geometry = Rc::new(Default::default());
    let material = Rc::new(Default::default());
    with_default_fields!(Mesh; geometry, material)
  }
}

impl Renderable for Line {
  fn geometry(&self) -> Rc<dyn IGeometry> {
    self.geometry.clone()
  }

  fn material(&self) -> Rc<dyn IMaterial> {
    self.material.clone()
  }
}

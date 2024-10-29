use std::rc::Rc;

use renderer_macro_derive::object_3d;

use crate::material::standard_material::StandardMeshMaterial;

use super::super::core::buffer_geometry::BufferGeometry;
use super::super::core::object_3d::{with_default_fields, ObjectActions};

#[object_3d(ObjectActions)]
pub struct Mesh {
  geometry: Rc<BufferGeometry>,
  material: Rc<StandardMeshMaterial>,
}

impl Mesh {
  pub fn new() -> std::rc::Rc<Self> {
    let geometry = Rc::new(Default::default());
    let material = Rc::new(Default::default());
    with_default_fields!(Mesh; geometry, material)
  }

  pub fn geometry(&self) -> Rc<BufferGeometry> {
    self.geometry.clone()
  }

  pub fn material(&self) -> Rc<StandardMeshMaterial> {
    self.material.clone()
  }
}

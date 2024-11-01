use std::rc::Rc;

use renderer_macro_derive::object_3d;

use crate::core::buffer_geometry::IGeometry;
use crate::material::material::IMaterial;
use crate::material::standard_material::StandardMeshMaterial;

use super::super::core::buffer_geometry::BufferGeometry;
use super::super::core::object_3d::{with_default_fields, IObject3D};
use super::base::Renderable;

#[object_3d(IObject3D)]
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
}

impl Renderable for Mesh {
  fn geometry(&self) -> Rc<dyn IGeometry> {
    self.geometry.clone()
  }

  fn material(&self) -> Rc<dyn IMaterial> {
    self.material.clone()
  }
}

use renderer_macro_derive::object_3d;

use crate::material::standard_material::StandardMeshMaterial;

use super::super::core::buffer_geometry::BufferGeometry;
use super::super::core::object_3d::{with_default_fields, ObjectActions};

#[object_3d(ObjectActions)]
pub struct Mesh {
  geometry: BufferGeometry,
  material: StandardMeshMaterial,
}

impl Mesh {
  pub fn new() -> std::rc::Rc<Self> {
    let geometry = Default::default();
    let material = Default::default();
    with_default_fields!(Mesh; geometry, material)
  }
}

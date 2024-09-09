use renderer_macro_derive::object_3d;

use super::super::material::material::BasicMaterial;

use super::super::core::buffer_geometry::BufferGeometry;
use super::super::core::object_3d::{
  /* define_support_objects, */ with_default_fields, ObjectActions,
};
// use super::group::Group;

// define_support_objects!(
//   MeshSupportChildren;
//   Mesh:Mesh,
//   Group:Group
// );

#[object_3d(ObjectActions)]
pub struct Mesh {
  geometry: BufferGeometry,
  material: BasicMaterial,
}

impl Mesh {
  pub fn new() -> Self {
    let geometry = Default::default();
    let material = Default::default();
    with_default_fields!(geometry, material)
  }
}

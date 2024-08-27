use super::super::material::material::BasicMaterial;

use super::super::core::buffer_geometry::BufferGeometry;

pub struct Mesh {
  geometry: BufferGeometry,
  material: BasicMaterial,
}

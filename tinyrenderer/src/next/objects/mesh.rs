use super::super::material::material::BasicMaterial;

use super::super::core::geometry::Geometry;

pub struct Mesh {
  geometry: Geometry,
  material: BasicMaterial,
}

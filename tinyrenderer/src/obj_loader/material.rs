use std::collections::HashMap;

use crate::math::Vec3;

pub struct Material {
  pub name: String,
  pub ambient: Option<Vec3>,
  pub diffuse: Option<Vec3>,
  pub specular: Option<Vec3>,
  pub emissive_coeficient: Option<Vec3>,
  pub specular_exponent: Option<f32>,
  pub dissolve: Option<f32>,
  pub transmission_filter: Option<Vec3>,
  pub optical_density: Option<f32>,
  pub illum: Option<u8>,
  pub texture_map: TexturePointer,
}

pub struct TexturePointer {
  pub ambient: Option<String>,
  pub diffuse: Option<String>,
  pub specular_color: Option<String>,
  pub specular_highlight: Option<String>,
  pub alpha: Option<String>,
  pub refl: Option<String>,
  pub bump: Option<String>,
}

pub struct Materials {
  pub materials: HashMap<String, Material>,
}

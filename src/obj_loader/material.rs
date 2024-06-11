use std::collections::HashMap;

use crate::math::{Vec2, Vec3};

// Material Library File
pub struct MtlTextureMaps {
  pub ambient: Option<String>,            // map_Ka path to Ka
  pub diffuse: Option<String>,            // map_Kd ...
  pub specular_color: Option<String>,     // map_Ks ...
  pub specular_highlight: Option<String>, // map_Ns ...
  pub alpha: Option<String>,              // map_d
  pub refl: Option<String>,               // map_refl
  pub bump: Option<String>,               // map_Bump  bump map (which by default uses luminance channel of the image)
}

/**
 * https://www.fileformat.info/format/material/
 */
#[rustfmt::skip]
pub struct Material {
  pub name: String,
  pub ambient: Option<Vec3>,                    // Ka in rgb and single value range from 0.0 to 1.0
  pub diffuse: Option<Vec3>,                    // Kd ...
  pub specular: Option<Vec3>,                   // Ks ...
  pub emissive_coeficient: Option<Vec3>,        // Ke ...
  pub specular_exponent: Option<f32>,           // Ns normally range from 0 to 1000.
  pub dissolve: Option<f32>,                    // d (default 1.0 -> opaque)
  // pub d_halo:Option<f32>,                       // d -halo,  dissolve = 1.0 - (N*v)(1.0-factor)
  pub transmission_filter: Option<Vec3>,        // Tf in rgb and single value range from 0.0 to 1.0
  pub optical_density: Option<f32>,             // Ni range from 0.001 to 10. (glass -> 1.5, affects the refraction)
  pub illum: Option<u8>,                        // illum 0 to 2

  pub texture_maps: MtlTextureMaps,
}

impl Material {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      ambient: None,
      diffuse: None,
      specular: None,
      emissive_coeficient: None,
      specular_exponent: None,
      dissolve: None,
      // d_halo: None,
      transmission_filter: None,
      optical_density: None,
      illum: None,
      texture_maps: MtlTextureMaps {
        ambient: None,
        diffuse: None,
        specular_color: None,
        specular_highlight: None,
        alpha: None,
        refl: None,
        bump: None,
      },
    }
  }
}

pub struct MtlLib {
  pub materials: HashMap<String, Material>,
}

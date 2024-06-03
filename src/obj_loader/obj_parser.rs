use std::{self, collections::HashMap, path::Path};

use crate::math::{Vec2, Vec3};

use super::{
  error::{Error, ParseResult},
  marcos::ignore_utils,
  token_requester::{TokenRequester, TokenType},
};

pub struct Vertex {
  pub vertex: u32,
  pub normal: Option<u32>,
  pub textcoord: Option<u32>,
}

pub struct Face {
  pub vertices: Vec<Vertex>,
}

pub struct Model {
  pub faces: Vec<Face>,
  pub name: String,
  pub mtllib: Option<u32>,
  pub material: Option<String>,
  pub smooth_shade: u8,
}

// Material Library File
pub struct MtlTextureMaps {
  pub ambient: Option<String>,            // map_Ka path to Ka
  pub diffuse: Option<String>,            // map_Kd ...
  pub specular_color: Option<String>,     // map_Ks ...
  pub specular_highlight: Option<String>, // map_Ns ...
  pub alpha: Option<String>,              // map_d
  pub refl: Option<String>,               // map_refl
  pub bump: Option<String>,               // map_Bump
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
  pub d_factor: Option<f32>,                    // d (default 1.0 -> opaque)
  pub d_halo:Option<f32>,                       // d -halo,  dissolve = 1.0 - (N*v)(1.0-factor)
  pub transmission_filter: Option<Vec3>,        // Tf in rgb and single value range from 0.0 to 1.0
  pub optical_density: Option<f32>,             // Ni range from 0.001 to 10. (glass -> 1.5, affects the refraction)
  pub illum: Option<u8>,                        // illum 0 to 2

  pub texture_maps: MtlTextureMaps,
}

impl Material {
  fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      ambient: None,
      diffuse: None,
      specular: None,
      emissive_coeficient: None,
      specular_exponent: None,
      d_factor: None,
      d_halo: None,
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

impl Default for Material {
  fn default() -> Self {
    Self::new("name")
  }
}

pub struct MtlLib {
  pub materials: HashMap<String, Material>,
}

#[derive(Default)]
pub struct SceneData {
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub textcoords: Vec<Vec2>,
  pub materials: Vec<MtlLib>,
  pub models: Vec<Model>,
}

impl SceneData {
  pub fn new() -> Self {
    Self {
      vertices: vec![],
      normals: vec![],
      textcoords: vec![],
      materials: vec![],
      models: vec![],
    }
  }
}

struct ObjParser<'a, 'b> {
  scene: SceneData,
  dirpath: &'a Path,
  requester: &'b mut TokenRequester<'b>,
}

impl<'a, 'b> ObjParser<'a, 'b> {
  pub fn new(path: &'a Path, requester: &'b mut TokenRequester<'b>) -> Self {
    Self {
      scene: SceneData::new(),
      dirpath: path,
      requester,
    }
  }

  pub fn parse(&mut self) -> ParseResult {
    let mut token = self.requester.request();
    let mut finish = false;
    while !finish {
      match token {
        TokenType::Token(str) => match str {
          "#" => todo!(),
          _ => return Err(Error::UnknownToken(str.to_string())),
        },
        TokenType::Nextline => {
          token = self.requester.request();
        }
        TokenType::Eof => {
          finish = true;
        }
      }
    }
    Ok(())
  }
}

// ignore_utils!()

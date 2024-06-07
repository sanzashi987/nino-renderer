use std::{collections::HashMap, ops::Not, path::Path};

use crate::math::{Vec2, Vec3};

use super::{
  error::{Error, ParseResult},
  file_content::FileContent,
  marcos::{parse_num, parse_token, skip_to_next_line},
  token_requester::{TokenRequester, TokenType},
  MtlLibParser,
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

pub struct ObjParser<'a, 'b> {
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
          // starts with # is comment, skip this line
          "#" => {
            skip_to_next_line!(token = self.requester.request();TokenType::Nextline, TokenType::Eof)
          }

          "g" | "o" => self.scene.models.push(Model {
            faces: vec![],
            name: parse_token!(token = self.requester.request(); String)?,
            mtllib: self
              .scene
              .materials
              .is_empty()
              .not()
              .then_some((self.scene.materials.len() - 1) as u32),
            material: None,
            smooth_shade: 0,
          }),
          // vertex defs
          "v" => self
            .scene
            .vertices
            .push(parse_token!(token = self.requester.request(); Vec3 = x:f32, y:f32, z:f32)?),
          // texture coordinates
          "vt" => self
            .scene
            .textcoords
            .push(parse_token!(token = self.requester.request(); Vec2 = x:f32 , y:f32)?),
          // normal
          "vn" => self
            .scene
            .normals
            .push(parse_token!(token = self.requester.request(); Vec3 = x:f32,y:f32,z:f32)?),
          // "f" =>
          "mtllib" => {
            token = self.requester.request();
            if let TokenType::Token(filename) = token {
              let mut path_buf =
                std::path::PathBuf::from(self.dirpath.parent().ok_or(Error::PathNotFound)?);
              path_buf.push(filename);

              let file_content = FileContent::from_file(path_buf.as_path())?;
              let mut mtllib_token_requester = TokenRequester::new(&file_content)?;
              let mut mtllib_parser = MtlLibParser::new(&mut mtllib_token_requester);

              self.scene.materials.push(mtllib_parser.parse()?);

              token = self.requester.request();
            }
          }
          "usemtl" => {
            self
              .scene
              .models
              .last_mut()
              .ok_or(Error::ParseIncomplete)?
              .material = Some(parse_token!(token = self.requester.request(); String)?)
          }
          "s" => {
            self
              .scene
              .models
              .last_mut()
              .ok_or(Error::ParseIncomplete)?
              .smooth_shade = parse_token!(token = self.requester.request();u8)?
          }
          // faces
          // Face with vertex only
          // => f v1 v2 v3
          // Face with vertex index and texture coordinate index
          // => f v1/vt1 v2/vt2 v3/vt3
          // Face with vertex index, texture coordinate index and vertex normal index
          // => f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
          // Face with vertex index and vertex normal index but not texture coordinate index
          // => f v1//vn1 v2//vn2 v3//vn3
          "f" => {
            token = self.requester.request();
            let mut vertices: Vec<Vertex> = Vec::new();

            let mut done = false;
            while !done {
              if let TokenType::Token(token_str) = token {
                let splited: Vec<&str> = token_str.split('/').collect();

                let indices = splited.as_slice();
                if indices.len() > 3 || indices.len() < 1 {
                  return Err(Error::InvalidSyntax);
                }
                let (mut textcoord, mut normal) = (None, None);

                match *indices {
                  [_, second, third] => {
                    if second.is_empty().not() {
                      textcoord = Some(parse_num!(second, u32) - 1);
                    }
                    normal = Some(parse_num!(third, u32) - 1);
                  }
                  [_, second] => {
                    textcoord = Some(parse_num!(second, u32) - 1);
                  }
                  _ => return Err(Error::InvalidSyntax),
                }

                let str = indices[0];
                let vertex = parse_num!(str, u32) - 1;

                vertices.push(Vertex {
                  vertex,
                  textcoord,
                  normal,
                })
              } else {
                done = true;
              }
              token = self.requester.request();
            }
            self
              .scene
              .models
              .last_mut()
              .ok_or(Error::ParseIncomplete)?
              .faces
              .push(Face { vertices });
          }
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
// fn a() {
//   <Vec3>::zero()
// }

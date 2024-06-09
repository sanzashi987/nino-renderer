use std::{ops::Not, path::Path};

use image::error;

use crate::math::{Vec2, Vec3};

use super::{
  error::{Error, ParseResult},
  face::{Face, Model, Vertex},
  file_content::FileContent,
  marcos::{parse_num, parse_token, skip_to_next_line},
  material::MtlLib,
  token_requester::{TokenRequester, TokenType},
  MtlLibParser,
};

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

pub fn load_from_file(filename: &str) -> Result<SceneData, Error> {
  let fullpath = std::path::Path::new(filename);
  match FileContent::from_file(fullpath) {
    Ok(content) => {
      let mut requester = TokenRequester::new(&content)?;
      let mut parser = ObjParser::new(fullpath, &mut requester);
      parser.parse()?;
      Ok(parser.scene)
    }
    Err(error) => return Err(Error::IoError(error)),
  }
}

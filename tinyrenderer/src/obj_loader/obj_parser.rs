use std::ops::Not;

use crate::math::{Vec2, Vec3};

use super::{
  defines::{self, parse_num, parse_token, ParserError},
  parser::{ParseLine, Parser},
  Face, Scene, VertexPointer,
};

pub struct ObjParserImpl;

impl ParseLine<Scene> for ObjParserImpl {
  fn parse_line(
    scene: &mut Scene,
    tokens: &mut std::str::SplitWhitespace,
    s: &str,
  ) -> Result<(), ParserError> {
    match s {
      "#" => {}
      "g" | "o" => {
        let name = parse_token!(tokens.next(); String)?;
        scene.add_model(name);
      }
      "v" => {
        scene.add_vertex(parse_token!(tokens.next(); Vec3 = x:f32, y:f32, z:f32)?);
      }
      "vn" => {
        scene.add_normal(parse_token!(tokens.next(); Vec3 = x:f32, y:f32, z:f32)?);
      }
      "vt" => {
        scene.add_texture_coordinate(parse_token!(tokens.next(); Vec2 = x:f32, y:f32)?);
      }
      "f" => {
        let mut vertex_vec = vec![];
        let mut done = false;

        while !done {
          match tokens.next() {
            Some(str) => {
              let splited: Vec<&str> = str.split("/").collect();
              let indices = splited.as_slice();
              if indices.len() > 3 || indices.len() < 1 {
                return Err(ParserError::InvalidSyntax(
                  "face vertex indices".to_string(),
                ));
              }

              let (mut texture_index, mut normal_index) = (None, None);

              match *indices {
                [_, second, third] => {
                  if second.is_empty().not() {
                    texture_index = Some(parse_num!(second, u32) - 1);
                  }
                  normal_index = Some(parse_num!(third, u32) - 1)
                }
                [_, second] => {
                  texture_index = Some(parse_num!(second, u32) - 1);
                }
                _ => return Err(ParserError::InvalidSyntax("face vertex format".to_string())),
              }

              let vertex_index = parse_num!(indices[0], u32) - 1;
              vertex_vec.push(VertexPointer::new(
                vertex_index,
                normal_index,
                texture_index,
              ));
            }
            None => {
              done = true;
            }
          }
        }
        if vertex_vec.len() != 3 {
          return Err(ParserError::InvalidSyntax("Face Vertices".to_string()));
        }

        let vertices: [VertexPointer; 3] = [vertex_vec[0], vertex_vec[1], vertex_vec[2]];
        scene.add_face(Face { vertices })?;
      }
      _ => {}
    };
    Ok(())
  }
}

pub type ObjParser<'a, 'b> = Parser<'a, 'b, Scene, ObjParserImpl>;

pub fn load_obj(relative_path: &str, mode: defines::ParserMode) -> Result<ObjParser, ParserError> {
  let fullpath = std::path::Path::new(relative_path);
  Ok(ObjParser::new(fullpath, mode)?)
}

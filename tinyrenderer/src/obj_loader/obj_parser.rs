use super::{model::Face, ParserError};
use crate::math::{Vec2, Vec3};
use std::{ops::Not, path::Path};

use super::{
  file_loader::FileLoader,
  model::{Model, Scene, VertexPointer},
};

pub struct ObjParser<'a, 'b> {
  scene: Scene,
  working_dir: &'a Path,
  loader: FileLoader<'b>,
  lazy: bool,
}
pub enum ParserMode {
  Standard,
  Lazy,
}

macro_rules! parse_num {
  ($var:ident,$type:ty) => {{
    $var
      .parse::<$type>()
      .map_err(|_| ParserError::CantConvertToNum)?
  }};
  ($exp:expr,$type:ty) => {{
    let val = $exp;
    val
      .parse::<$type>()
      .map_err(|_| ParserError::CantConvertToNum)?
  }};
}

macro_rules! parse_token {
    ($iter: expr; $type:ty) => {{
      let result = if let Some(s) = $iter {
        s
          .parse::<$type>()
          .map_err(|_| ParserError::CantConvertToType)
      } else {
        Err(ParserError::ParseIncomplete("not a valiad string".to_string()))
      };
      // move to next token
      result
    }};

    ($iter: expr, $type:ty = $($attr:ident : $attr_type:ty),+) => {
      {
        let mut val = <$type>::zero();
        $(
          if let Some(s) = $iter {
            val.$attr = parse_num!(s, $attr_type);
          } else {
            return Err(ParserError::UnExpectedEndOfLine)
          }
        )+

        Ok::<$type,ParserError>(val)
      }
    };
}

impl<'a, 'b> ObjParser<'a, 'b>
where
  'a: 'b,
{
  pub fn new(filepath: &'a Path, mode: ParserMode) -> Result<Self, ParserError> {
    let parent = filepath.parent();
    let filename = filepath.file_name();
    let loader_result: Result<FileLoader<'b>, std::io::Error> = FileLoader::new(filepath);

    let (lazy, single) = match mode {
      ParserMode::Standard => (false, true),
      ParserMode::Lazy => (true, false),
    };

    if let (Some(dir), Some(name)) = (parent, filename) {
      match loader_result {
        Ok(loader) => {
          let mut parser = Self {
            working_dir: dir,
            loader: loader,
            scene: Scene::new(),
            lazy,
          };

          if !lazy {
            parser.parse()?
          }

          return Ok(parser);
        }
        Err(e) => return Err(ParserError::IoError(e)),
      }
    };
    Err(ParserError::NotAValidPath)
  }

  fn parse(&mut self) -> Result<(), ParserError> {
    if self.loader.is_done() {
      return Ok(());
    }

    for line in &mut self.loader {
      let trimmed = line.trim().to_string();
      let mut tokens = trimmed.split_whitespace();

      let token = tokens.next();
      if let Some(s) = token {
        match s {
          "#" => {
            continue;
          }
          "g" | "o" => {
            let name = parse_token!(tokens.next(); String)?;
          }
          "v" => {
            self
              .scene
              .add_vertex(parse_token!(tokens.next(), Vec3 = x:f32, y:f32, z:f32)?);
          }
          "vn" => {
            self
              .scene
              .add_normal(parse_token!(tokens.next(), Vec3 = x:f32, y:f32, z:f32)?);
          }
          "vt" => {
            self
              .scene
              .add_texture_coordinate(parse_token!(tokens.next(), Vec2 = x:f32, y:f32)?);
          }
          "f" => {
            // let mut vertex_pointer = VertexPointer {};
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
                  // let str = indices[0];
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
              // return Err(ParserError::)
            }

            let vertices: [VertexPointer; 3] = [vertex_vec[0], vertex_vec[1], vertex_vec[2]];
            self.scene.add_face(Face { vertices })?;
          }
          _ => {
            continue;
            // _ => return Err(ParserError::UnknownToken(s.to_string())),
          }
        }
      }
    }

    // Ok(&self.result)
    Ok(())
  }

  pub fn get_result(&mut self) -> Result<&Scene, ParserError> {
    self.parse()?;

    Ok(&self.scene)
  }
}

pub fn load_obj(relative_path: &str, mode: ParserMode) -> Result<ObjParser, ParserError> {
  let fullpath = std::path::Path::new(relative_path);
  Ok(ObjParser::new(fullpath, mode)?)
}

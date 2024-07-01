use std::path::Path;

use crate::math::{Vec2, Vec3};

use super::{file_loader::FileLoader, model::Model};

pub struct ObjParser<'a, 'b> {
  result: Vec<Model>,
  working_dir: &'a Path,
  loader: FileLoader<'b>,
}

pub enum ParserError {
  IoError(std::io::Error),
  NotAValidPath,
  InvalidSyntax,
  ParseIncomplete,
  UnknownToken(String),
  CantConvertToNum,
}

macro_rules! parse_num {
  ($var:ident,$type:ty) => {{
    $var
      .parse::<$type>()
      .map_err(|_| ParserError::CantConvertToNum)?
  }};
}

macro_rules! parse_token {
    ($iter: expr, $type:ty = $($attr:ident : $attr_type:ty),+) => {
      {
        let mut val = <$type>::zero();
        $(
          if let Some(s) = $iter {
            val.$attr = parse_num!(s, $attr_type);
          } else {
            return Err(ParserError::ParseIncomplete)
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
  pub fn new(filepath: &'a Path, single_mode: bool) -> Result<Self, ParserError> {
    let parent = filepath.parent();
    let filename = filepath.file_name();
    let loader_result: Result<FileLoader<'b>, std::io::Error> = FileLoader::new(filepath);

    if let (Some(dir), Some(name)) = (parent, filename) {
      match loader_result {
        Ok(loader) => {
          return Ok(Self {
            working_dir: dir,
            loader: loader,
            result: if name.to_str().is_some() && single_mode {
              vec![Model::single_mode(name.to_str().unwrap())]
            } else {
              Vec::new()
            },
          });
        }
        Err(e) => return Err(ParserError::IoError(e)),
      }
    };
    Err(ParserError::NotAValidPath)
  }

  pub fn parse(&mut self) -> Result<&Vec<Model>, ParserError> {
    if self.loader.is_done() {
      return Ok(&self.result);
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
          "v" => {
            self
              .result
              .last_mut()
              .ok_or(ParserError::ParseIncomplete)?
              .add_vertex(parse_token!(tokens.next(), Vec3 = x:f32, y:f32, z:f32)?);
          }
          "vn" => {
            self
              .result
              .last_mut()
              .ok_or(ParserError::ParseIncomplete)?
              .add_normal(parse_token!(tokens.next(), Vec3 = x:f32, y:f32, z:f32)?);
          }
          "vt" => {
            self
              .result
              .last_mut()
              .ok_or(ParserError::ParseIncomplete)?
              .add_texture_coordinate(parse_token!(tokens.next(), Vec2 = x:f32, y:f32)?);
          }
          _ => {}
        }
      }
    }

    Ok(&self.result)
  }
}

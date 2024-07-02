use std::path::Path;

use crate::math::{Vec2, Vec3};

use super::{
  file_loader::FileLoader,
  model::{Model, Scene},
};

pub struct ObjParser<'a, 'b> {
  scene: Scene,
  // empty: Vec<Model>,
  working_dir: &'a Path,
  loader: FileLoader<'b>,
  lazy: bool,
}
#[derive(Debug)]
pub enum ParserError {
  IoError(std::io::Error),
  NotAValidPath,
  InvalidSyntax,
  ParseIncomplete(String),
  UnknownToken(String),
  CantConvertToNum,
  UnExpectedEndOfLine,
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
}

macro_rules! parse_token {
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

  pub fn get_result(&mut self) -> &Scene {
    let res = self.parse();

    if res.is_ok() {
      &self.scene
    } else {
      &self.empty
    }
  }
}

pub fn load_obj(relative_path: &str, mode: ParserMode) -> Result<ObjParser, ParserError> {
  let fullpath = std::path::Path::new(relative_path);
  Ok(ObjParser::new(fullpath, mode)?)
}

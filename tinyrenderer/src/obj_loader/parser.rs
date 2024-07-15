use super::{
  defines::{self, ParserError},
  model::Face,
};
use crate::math::{Vec2, Vec3};
use std::{borrow::BorrowMut, marker::PhantomData, ops::Not, path::Path};

use super::{
  file_loader::FileLoader,
  model::{Model, Scene, VertexIndex},
};

pub trait ParseLine {
  fn parse_line(&mut self, s: &str) -> Result<(), ParserError>;
}

pub struct Parser<'a, 'b, Data: Default, Abstracts: ParseLine> {
  data: Data,
  filepath: &'a Path,
  loader: Option<FileLoader<'b>>,
  lazy: bool,
  _phantom: PhantomData<Abstracts>,
}

impl<'a, 'b, Data, Abstracts> Parser<'a, 'b, Data, Abstracts>
where
  'a: 'b,
  Data: Default,
  Abstracts: ParseLine,
{
  pub fn new(filepath: &'a Path, mode: defines::ParserMode) -> Result<Self, ParserError> {
    let lazy = match mode {
      defines::ParserMode::Lazy => true,
      _ => false,
    };

    let mut parser = Self {
      filepath,
      loader: None,
      data: Default::default(),
      lazy,
      _phantom: PhantomData,
    };

    if !lazy {
      parser.parse()?
    }

    return Ok(parser);
  }

  fn parse(&mut self) -> Result<(), ParserError> {
    if let None = self.loader {
      let loader: Result<FileLoader<'b>, ParserError> =
        FileLoader::new(self.filepath).map_err(|e| ParserError::IoError(e));
      self.loader = Some(loader?);
    }

    let loader = self.loader.as_mut().unwrap();

    if loader.is_done() {
      return Ok(());
    }

    for line in loader {
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
            self.scene.add_model(name);
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
                  vertex_vec.push(VertexIndex::new(
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

            let vertices: [VertexIndex; 3] = [vertex_vec[0], vertex_vec[1], vertex_vec[2]];
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

  pub fn get_result(&mut self) -> Result<&mut Data, ParserError> {
    self.parse()?;

    Ok(&mut self.data)
  }
}

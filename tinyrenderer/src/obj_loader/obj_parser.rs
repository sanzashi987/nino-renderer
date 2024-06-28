use std::path::Path;

use super::{file_loader::FileLoader, model::Model};

pub struct ObjParser<'a, 'b> {
  result: Vec<Model>,
  current_model: Option<Model>,
  working_dir: &'a Path,
  loader: FileLoader<'b>,
}

pub enum ParserError {
  IoError(std::io::Error),
  NotAValidPath,
  InvalidSyntax,
  ParseIncomplete,
  UnknownToken(String),
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
            result: Vec::new(),
            current_model: if name.to_str().is_some() && single_mode {
              Some(Model::single_mode(name.to_str().unwrap()))
            } else {
              None
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
          "v" => {}
          "vn" => {}
          "vt" => {}
          _ => {}
        }
      }
    }

    Ok(&self.result)
  }
}

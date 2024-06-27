use std::path::Path;

use super::{file_loader::FileLoader, model::Model};

pub struct ObjParser<'a, 'b> {
  result: Vec<Model>,
  current_model: Option<Model>,
  working_dir: &'a Path,
  loader: FileLoader<'b>,
}

impl<'a, 'b> ObjParser<'a, 'b>
where
  'a: 'b,
{
  pub fn new(filename: &'a Path) -> Self {
    let working_dir = filename.parent().unwrap();
    let loader: FileLoader<'b> = FileLoader::new(filename).unwrap();

    Self {
      working_dir,
      loader: loader,
      result: Vec::new(),
      current_model: None,
    }
  }
}

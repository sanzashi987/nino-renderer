// mtl -> Material Template Library
use super::{
  file_loader::FileLoader,
  model::{Model, Scene, VertexPointer},
  obj_parser::ParserMode,
  ParserError,
};

pub struct MtlParser<'a> {
  loader: FileLoader<'a>,
  lazy: bool,
}

impl<'a> MtlParser<'a> {
  pub fn new(filepath: &std::path::Path, mode: ParserMode) -> Result<Self, ParserError> {
    let parent = filepath.parent();
    let filename = filepath.file_name();
    let loader_result: Result<FileLoader<'a>, std::io::Error> = FileLoader::new(filepath);

    let (lazy, single) = match mode {
      ParserMode::Standard => (false, true),
      ParserMode::Lazy => (true, false),
    };

    if let (Some(dir), Some(name)) = (parent, filename) {
      match loader_result {
        Ok(loader) => {
          let mut this = Self {
            loader: loader,
            lazy,
          };

          if !lazy {
            this.parse()?
          }

          return Ok(this);
        }
        Err(e) => return Err(ParserError::IoError(e)),
      }
    };
    Err(ParserError::NotAValidPath)
  }

  pub fn parse(&self) {}
}

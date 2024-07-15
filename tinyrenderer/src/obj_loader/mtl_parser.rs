// mtl -> Material Template Library
use super::{
  error::{self, ParserError},
  file_loader::FileLoader,
  model::{Model, Scene, VertexPointer},
  obj_parser::ParserMode,
};

pub struct MtlParser<'a> {
  loader: FileLoader<'a>,
  lazy: bool,
}

impl<'a, 's> MtlParser<'a>
where
  's: 'a,
{
  pub fn new(filepath: &'s std::path::Path, mode: ParserMode) -> Result<Self, ParserError> {
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

  pub fn parse(&mut self) -> Result<(), ParserError> {
    if self.loader.is_done() {
      return Ok(());
    }

    error::parse(&mut self.loader, |s| match s {
      "#" | _ => Ok(()),
    })
  }
}

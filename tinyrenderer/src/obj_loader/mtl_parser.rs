// mtl -> Material Template Library
use super::{
  defines::{self, ParserError},
  file_loader::FileLoader,
  material::Materials,
  model,
};

pub struct MtlParser<'a> {
  mtl: Materials,
  loader: FileLoader<'a>,
}

impl<'a, 's> MtlParser<'a>
where
  's: 'a,
{
  pub fn new(
    filepath: &'s std::path::Path,
    mode: defines::ParserMode,
  ) -> Result<Self, ParserError> {
    let parent = filepath.parent();
    let filename = filepath.file_name();
    let loader_result: Result<FileLoader<'a>, std::io::Error> = FileLoader::new(filepath);

    if let (Some(dir), Some(name)) = (parent, filename) {
      match loader_result {
        Ok(loader) => {
          let mut this = Self {
            mtl: Materials::new(),
            loader: loader,
          };
          return Ok(this);
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

    model::parse(&mut self.loader, |s| match s {
      "newmtl" => {
        todo!()
      }
      "#" | _ => Ok(()),
    })
  }
}

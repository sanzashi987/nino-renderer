use super::defines::{self, ParserError};
use std::{marker::PhantomData, path::Path};

use super::{
  file_loader::FileLoader,
  model::{Model, Scene, VertexIndex},
};

pub trait ParseLine<Data: Default> {
  fn parse_line(
    data: &mut Data,
    tokens: &mut std::str::SplitWhitespace,
    s: &str,
  ) -> Result<(), ParserError>;
}

pub struct Parser<'a, 'b, Data: Default, Abstracts: ParseLine<Data>> {
  data: Data,
  filepath: &'a Path,
  loader: Option<FileLoader<'b>>,
  // lazy: bool,
  _phantom: PhantomData<Abstracts>,
}
impl<'a, 'b, Data, Abstracts> Parser<'a, 'b, Data, Abstracts>
where
  'a: 'b,
  Data: Default,
  Abstracts: ParseLine<Data>,
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
        Abstracts::parse_line(&mut self.data, &mut tokens, s)?;
      }
    }

    // Ok(&self.result)
    Ok(())
  }

  pub fn get_result(&mut self) -> Result<&mut Data, ParserError> {
    self.parse()?;

    Ok(&mut self.data)
  }

  pub fn get_data_mut(&mut self) -> &mut Data {
    &mut self.data
  }

  pub fn get_data_own(self) -> Data {
    self.data
  }
}

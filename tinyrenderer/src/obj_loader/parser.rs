use super::defines::{ParserError, ParserResult};
use std::{marker::PhantomData, path::Path};

use super::file_loader::FileLoader;

pub trait ParseLine<Data: Default> {
  fn parse_line(
    data: &mut Data,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    s: &str,
  ) -> ParserResult;
}

pub struct Parser<'a, 'b, Data: Default, Abstracts: ParseLine<Data>> {
  data: Data,
  filepath: &'a Path,
  working_dir: &'a str,
  loader: Option<FileLoader<'b>>,
  _phantom: PhantomData<Abstracts>,
}
impl<'a, 'b, Data, Abstracts> Parser<'a, 'b, Data, Abstracts>
where
  'a: 'b,
  Data: Default,
  Abstracts: ParseLine<Data>,
{
  pub fn new(filepath: &'a Path) -> Result<Self, ParserError> {
    let working_dir = filepath
      .parent()
      .unwrap()
      .to_str()
      .expect("Not a valid working dir str");
    Ok(Self {
      filepath,
      working_dir,
      loader: None,
      data: Default::default(),
      _phantom: PhantomData,
    })
  }

  fn parse(&mut self) -> ParserResult {
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
        Abstracts::parse_line(&mut self.data, &mut tokens, self.working_dir, s)?;
      }
    }

    Ok(())
  }

  pub fn get_result(&mut self) -> Result<&mut Data, ParserError> {
    self.parse()?;

    Ok(&mut self.data)
  }
}

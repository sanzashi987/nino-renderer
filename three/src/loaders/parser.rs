use std::{collections::HashMap, marker::PhantomData, path::Path};

use crate::utils::swap_and_move;

use super::{
  defines::{ParserError, ParserResult},
  file_loader::FileLoader,
};

pub trait ParseLine<Data: Default> {
  fn parse_line(
    data: &mut Data,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    token_str: &str,
  ) -> ParserResult;
}

pub struct Parser<Data: Default, Abstracts: ParseLine<Data>> {
  data: Data,
  _phantom: PhantomData<Abstracts>,
  loader: FileLoader,
  filepath: String,
  working_dir: String,
}

impl<Data, Abstracts> Parser<Data, Abstracts>
where
  Data: Default,
  Abstracts: ParseLine<Data>,
{
  pub fn new(path: &str) -> Result<Self, ParserError> {
    let working_dir: String = Path::new(path)
      .parent()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    let filepath = path.to_string();
    let loader = FileLoader::new(filepath.clone())?;
    Ok(Self {
      filepath,
      working_dir,
      loader,
      data: Default::default(),
      _phantom: PhantomData,
    })
  }

  fn _parse(&mut self) -> ParserResult {
    if self.loader.is_done() {
      return Ok(());
    }

    for line in &mut self.loader {
      let trimmed = line.trim().to_string();
      let mut tokens = trimmed.split_whitespace();

      let token = tokens.next();
      if let Some(token_str) = token {
        Abstracts::parse_line(&mut self.data, &mut tokens, &self.working_dir, token_str)?;
      }
    }
    Ok(())
  }

  pub fn parse(&mut self) -> Result<Data, ParserError> {
    self._parse()?;
    Ok(swap_and_move(&mut self.data))
  }
}
#[derive(Debug, Default)]
pub struct Loader<T: Default> {
  pub(super) next_id: u32,
  pub(super) loaded: HashMap<u32, T>,
  pub(super) path_id_map: HashMap<String, u32>,
}

impl<T: Default> Loader<T> {
  
}

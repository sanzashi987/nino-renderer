use std::{collections::HashMap, marker::PhantomData, path::Path};

use super::{
  defines::{ParserError, ParserResult},
  file_loader::FileLoader,
};

pub trait AssignId {
  fn assign_id(&mut self, id: u32) {}
}

pub trait Parse<Data: Default + AssignId> {
  fn parse(path: &str, id: u32) -> Result<Data, ParserError> {
    let working_dir = Path::new(path)
      .parent()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    let filepath = path.to_string();
    let mut loader = FileLoader::new(filepath.clone())?;

    let mut data = Data::default();

    for line in &mut loader {
      let trimmed = line.trim().to_string();
      let mut tokens = trimmed.split_whitespace();

      let token = tokens.next();
      if let Some(token_str) = token {
        Self::parse_line(&mut data, &mut tokens, &working_dir, token_str)?;
      }
    }

    Ok(data)
  }
  fn parse_line(
    data: &mut Data,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    token_str: &str,
  ) -> ParserResult;
}

#[derive(Debug)]
pub struct Loader<Data: Default + AssignId, Abstracts: Parse<Data>> {
  pub(super) next_id: u32,
  pub(super) loaded: HashMap<u32, Data>,
  pub(super) path_id_map: HashMap<String, u32>,
  _impls: PhantomData<Abstracts>,
}

impl<Data, Abstracts> Loader<Data, Abstracts>
where
  Data: Default + AssignId,
  Abstracts: Parse<Data>,
{
  pub fn insert_data(&mut self, mut data: Data, filepath: &str) -> u32 {
    let uid = self.next_id;
    data.assign_id(uid);

    self.loaded.insert(self.next_id, data);
    self.path_id_map.insert(filepath.to_string(), self.next_id);
    self.next_id += 1;
    uid
  }

  pub fn load(&mut self, filepath: &str) -> Result<&Data, ParserError> {
    if let Some(data_uid) = self.path_id_map.get(filepath) {
      return self
        .loaded
        .get(data_uid)
        .ok_or(ParserError::LoaderInstanceLoss);
    }

    let mut data = Abstracts::parse(filepath, self.next_id)?;
    let uid = self.insert_data(data, filepath);

    Ok(self.loaded.get(&uid).unwrap())
  }
}

impl<Data, Abstracts> Default for Loader<Data, Abstracts>
where
  Data: Default + AssignId,
  Abstracts: Parse<Data>,
{
  fn default() -> Self {
    Self {
      next_id: Default::default(),
      loaded: Default::default(),
      path_id_map: Default::default(),
      _impls: Default::default(),
    }
  }
}

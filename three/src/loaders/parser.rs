use std::{collections::HashMap, marker::PhantomData, path::Path};

use super::{
  defines::{ParserError, ParserResult},
  file_loader::FileLoader,
};

pub trait AssignId {
  fn assign_id(&mut self, id: u32) {}
}

pub enum SingleOrList<T> {
  Data(T),
  List(Vec<T>),
}
pub trait Parse<Data: Default + AssignId> {
  fn parse(full_path: &str, id: u32) -> Result<SingleOrList<Data>, ParserError> {
    let working_dir = Path::new(full_path)
      .parent()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    let filepath = full_path.to_string();
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
  ) -> ParserResult {
    Ok(())
  }
}

#[derive(Debug)]
pub struct Loader<Data: Default + AssignId, Abstracts: Parse<Data>> {
  pub(super) next_id: u32,
  pub(super) loaded: HashMap<u32, Data>,
  pub(super) name_id_map: HashMap<String, u32>,
  _impls: PhantomData<Abstracts>,
}

impl<Data, Abstracts> Loader<Data, Abstracts>
where
  Data: Default + AssignId,
  Abstracts: Parse<Data>,
{
  fn store_to_loaded(&mut self, mut data: Data, filepath: &str) -> u32 {
    let uid = self.next_id;
    data.assign_id(uid);

    self.loaded.insert(self.next_id, data);
    self.name_id_map.insert(filepath.to_string(), self.next_id);
    self.next_id += 1;
    uid
  }

  pub fn load(&mut self, filepath: &str) -> Result<&Data, ParserError> {
    if let Some(data_uid) = self.name_id_map.get(filepath) {
      return self
        .loaded
        .get(data_uid)
        .ok_or(ParserError::LoaderInstanceLoss);
    }

    let mut mixed_result = Abstracts::parse(filepath, self.next_id)?;

    match mixed_result {
      SingleOrList::Data(mut data) => {
        let uid = self.store_to_loaded(data, filepath);
        return Ok(self.loaded.get(&uid).unwrap());
      }
      SingleOrList::List(mut list) => {
        for data in &mut list {
          self.store_to_loaded(data, filepath);
        }
      }
    }
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
      name_id_map: Default::default(),
      _impls: Default::default(),
    }
  }
}

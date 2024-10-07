use std::{collections::HashMap, marker::PhantomData, path::Path};

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

pub trait AssignId {
  fn assign_id(&mut self, id: u32);
}
#[derive(Debug, Default)]
pub struct Loader<Data: Default + AssignId, Abstracts: ParseLine<Data>> {
  pub(super) next_id: u32,
  pub(super) loaded: HashMap<u32, Data>,
  pub(super) path_id_map: HashMap<String, u32>,
  // parser: Parser,
  _impls: PhantomData<Abstracts>,
}

impl<Data, Abstracts> Loader<Data, Abstracts>
where
  Data: Default + AssignId,
  Abstracts: ParseLine<Data>,
{
  pub fn insert_data(&mut self, mut data: Data, filepath: &str) -> u32 {
    let uid = self.next_id;
    data.assign_id(uid);

    self.loaded.insert(self.next_id, data);
    self.path_id_map.insert(filepath.to_string(), self.next_id);
    self.next_id += 1;
    uid
  }

  pub fn if_exist(&self, filepath: &str) -> Option<&Data> {
    if let Some(data_uid) = self.path_id_map.get(filepath) {
      self.loaded.get(data_uid)
    } else {
      None
    }
  }

  pub fn load(&mut self, filepath: &str) -> Result<&Data, ParserError> {
    if let Some(data) = self.if_exist(filepath) {
      return Ok(data);
    }

    let mut data = self.parse(filepath)?;
    let uid = self.insert_data(data, filepath);

    Ok(self.loaded.get(&uid).unwrap())
  }

  fn parse(&mut self, path: &str) -> Result<Data, ParserError> {
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
        Abstracts::parse_line(&mut data, &mut tokens, &working_dir, token_str)?;
      }
    }

    Ok(data)
  }
}

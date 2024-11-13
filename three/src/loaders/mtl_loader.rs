use std::{any::Any, collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::{math::Vec3, utils::SingleOrList};

use super::{
  defines::{parse_token, parse_token_ok, ParserError},
  parser::{ILoaderData, Loader, Parse},
  texture_loader::texture_loader,
};

#[derive(Debug, Default)]
pub struct MtlData {
  pub uid: u32,
  pub name: String,
  pub attributes: HashMap<String, Box<dyn Any + Send>>,
  pub textures: HashMap<String, String>,
}

impl MtlData {
  pub fn get_attr(&self, key: &str) -> Option<&Box<dyn Any + Send>> {
    self.attributes.get(key)
  }
}

impl ILoaderData for MtlData {
  fn assign_id(&mut self, id: u32) {
    self.uid = id;
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }
}

pub struct MtlParserImpl;

macro_rules! assign_last_mtl {
  ($data: ident; $iter: ident; $key:tt; $type:ty) => {
    {
      let val = parse_token!($iter.next();$type)?;
      let to_insert: Box<dyn Any + Send> = Box::new(val);
      $data
        .last_mut()
        .ok_or(ParserError::MtlNotFound)?
        .attributes
        .insert($key.to_string(), to_insert);
    }
  };
  ($data: ident; $iter: ident; $key:tt; $type:ty = $($attr:ident : $attr_type:ty),+) => {
     {
      let val = parse_token!($iter.next();$type = $($attr: $attr_type),+)?;
      let to_insert: Box<dyn Any + Send> = Box::new(val);
      $data
        .last_mut()
        .ok_or(ParserError::MtlNotFound)?
        .attributes
        .insert($key.to_string(), to_insert);
    }
  };
}

macro_rules! assign_last_mtl_texture {
  ($data: ident; $iter: ident; $key: tt;$dir: ident) => {
    {
      let texture  =parse_token!($iter.next();String)?;
      let texture = Self::append_to_working_dir($dir,&texture)?;
      $data.last_mut().ok_or(ParserError::MtlNotFound)?.textures.insert($key.to_string(),texture);
    }
  };
}

impl Parse<MtlData> for MtlParserImpl {
  fn init_data() -> SingleOrList<MtlData> {
    SingleOrList::List(vec![])
  }

  fn parse_line(
    data: &mut SingleOrList<MtlData>,
    tokens: &mut std::str::SplitWhitespace,
    fullpath: &str,
    token_str: &str,
  ) -> super::defines::ParserResult {
    if let SingleOrList::List(data) = data {
      let working_dir = Self::get_working_dir(fullpath)?;

      match token_str {
        "#" => {}
        "newmtl" => {
          let name = parse_token!(tokens.next();String)?;
          let mut mtl = MtlData::default();
          mtl.name = format!("{}@{}", fullpath, &name);
          data.push(mtl);
        }
        "Ns" => assign_last_mtl!(data;tokens;"Ns";f32),
        "Ka" => assign_last_mtl!(data;tokens;"Ka";Vec3=x:f32,y:f32,z:f32),
        "Kd" => assign_last_mtl!(data;tokens;"Kd";Vec3=x:f32,y:f32,z:f32),
        "Ks" => assign_last_mtl!(data;tokens;"Ks";Vec3=x:f32,y:f32,z:f32),
        "Ke" => assign_last_mtl!(data;tokens;"Ke";Vec3=x:f32,y:f32,z:f32),
        "Tf" => assign_last_mtl!(data;tokens;"Tf";Vec3=x:f32,y:f32,z:f32),
        "Ni" => assign_last_mtl!(data;tokens;"Ni";f32),
        "d" => assign_last_mtl!(data;tokens;"d";f32),
        "Tr" => assign_last_mtl!(data;tokens;"Tr";f32),
        "illum" => assign_last_mtl!(data;tokens;"illum";u32),
        "map_Ka" => assign_last_mtl_texture!(data;tokens;"map_Ka";working_dir),
        "map_Kd" => assign_last_mtl_texture!(data;tokens;"map_Kd";working_dir),
        "map_Ks" => assign_last_mtl_texture!(data;tokens;"map_Ks";working_dir),
        "map_Ns =>" => assign_last_mtl_texture!(data;tokens;"map_Ns";working_dir),
        "map_d" => assign_last_mtl_texture!(data;tokens;"map_d";working_dir),
        "map_refl" => assign_last_mtl_texture!(data;tokens;"map_refl";working_dir),
        "map_Bump" => assign_last_mtl_texture!(data;tokens;"map_Bump";working_dir),

        "norm" => assign_last_mtl_texture!(data;tokens;"norm";working_dir),
        _ => {}
      }
    }
    Ok(())
  }
  fn on_loaded(data: &MtlData) -> super::defines::ParserResult {
    let mut texture_mut_loader = texture_loader.lock().unwrap();

    for (_, path) in &data.textures {
      texture_mut_loader.load(path)?;
    }
    Ok(())
  }
}

type MtlLoader = Loader<MtlData, MtlParserImpl>;

lazy_static! {
  pub static ref mtl_loader: Mutex<MtlLoader> = Mutex::new(Default::default());
}

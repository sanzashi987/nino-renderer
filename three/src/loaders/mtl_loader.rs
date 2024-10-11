use std::{
  collections::HashMap,
  iter::Map,
  path::{Path, PathBuf},
};

use lazy_static::lazy_static;

use crate::{math::Vec3, utils::SingleOrList};

use super::{
  defines::{parse_token, parse_token_ok, ParserError},
  parser::{ILoaderData, Loader, Parse},
  texture_loader::texture_loader,
};

#[derive(Debug, Default)]
struct MtlData {
  uid: u32,
  name: String,
  ambient: Option<Vec3>,
  diffuse: Option<Vec3>,
  specular: Option<Vec3>,
  emissive_coeficient: Option<Vec3>,
  specular_exponent: Option<f32>,
  dissolve: Option<f32>,
  transmission_filter: Option<Vec3>,
  optical_density: Option<f32>,
  receive_shadow: Option<bool>,
  illum: Option<u8>,
  textures: HashMap<String, String>,
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

macro_rules! parse_texture_token {
  ($expr:expr; $textures:ident; $dir:ident) => {
    {
      let name = parse_token_ok!($expr;String);
      if let Some(n) = &name {
        let mut filepath = $dir.to_string();
        filepath.push_str(&n);

        let _ = $textures.load(&filepath, n);
      }
      name
    }
  };
}

macro_rules! assign_last_mtl {
  ($data: ident; $iter: ident; $key:tt; $type:ty) => {
    $data.last_mut().ok_or(ParserError::MtlNotFound)?.$key = parse_token_ok!($iter.next();$type);
  };
  ($data: ident; $iter: ident; $key:tt; $type:ty = $($attr:ident : $attr_type:ty),+) => {
    $data.last_mut().ok_or(ParserError::MtlNotFound)?.$key = parse_token_ok!($iter.next();$type = $($attr: $attr_type),+);
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
        "Ns" => assign_last_mtl!(data;tokens;specular_exponent;f32),
        "Ka" => assign_last_mtl!(data;tokens;ambient;Vec3=x:f32,y:f32,z:f32),
        "Kd" => assign_last_mtl!(data;tokens;diffuse;Vec3=x:f32,y:f32,z:f32),
        "Ks" => assign_last_mtl!(data;tokens;specular;Vec3=x:f32,y:f32,z:f32),
        "Ke" => assign_last_mtl!(data;tokens;emissive_coeficient;Vec3=x:f32,y:f32,z:f32),
        "Tf" => assign_last_mtl!(data;tokens;transmission_filter;Vec3=x:f32,y:f32,z:f32),
        "Ni" => assign_last_mtl!(data;tokens;optical_density;f32),
        "d" => assign_last_mtl!(data;tokens;dissolve;f32),
        "Tr" => assign_last_mtl!(data;tokens;dissolve;f32),
        "illum" => assign_last_mtl!(data;tokens;illum;u8),
        "map_Ka" => {}
        _ => {}
      }
    }
    Ok(())
  }
}

type MtlLoader = Loader<MtlData, MtlParserImpl>;

lazy_static! {
  pub static ref mtl_loader: MtlLoader = Default::default();
}

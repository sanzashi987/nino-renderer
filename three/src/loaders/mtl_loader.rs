use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::{math::Vec3, utils::SingleOrList};

use super::{
  defines::{parse_token, parse_token_ok, ParserError},
  parser::{ILoaderData, Loader, Parse},
  texture_loader::texture_loader,
};

#[derive(Debug, Default)]
pub struct MtlData {
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

macro_rules! assign_last_mtl {
  ($data: ident; $iter: ident; $key:tt; $type:ty) => {
    $data.last_mut().ok_or(ParserError::MtlNotFound)?.$key = parse_token_ok!($iter.next();$type);
  };
  ($data: ident; $iter: ident; $key:tt; $type:ty = $($attr:ident : $attr_type:ty),+) => {
    $data.last_mut().ok_or(ParserError::MtlNotFound)?.$key = parse_token_ok!($iter.next();$type = $($attr: $attr_type),+);
  };
}

macro_rules! assign_last_mtl_texture {
  ($data: ident; $iter: ident;$dir: ident; $str: tt) => {
    {
      let texture  =parse_token!($iter.next();String)?;
      let texture = Self::append_to_working_dir($dir,&texture)?;
      $data.last_mut().ok_or(ParserError::MtlNotFound)?.textures.insert($str.to_string(),texture);
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
        "map_Ka" => assign_last_mtl_texture!(data;tokens;working_dir;"map_Ka"),
        "map_Kd" => assign_last_mtl_texture!(data;tokens;working_dir;"map_Kd"),
        "map_Ks" => assign_last_mtl_texture!(data;tokens;working_dir;"map_Ks"),
        "map_Ns =>" => assign_last_mtl_texture!(data;tokens;working_dir;"map_Ns"),
        "map_d" => assign_last_mtl_texture!(data;tokens;working_dir;"map_d"),
        "map_refl" => assign_last_mtl_texture!(data;tokens;working_dir;"map_refl"),
        "map_Bump" => assign_last_mtl_texture!(data;tokens;working_dir;"map_Bump"),
        "norm" => assign_last_mtl_texture!(data;tokens;working_dir;"norm"),
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

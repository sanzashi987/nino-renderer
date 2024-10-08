use std::path::PathBuf;

use lazy_static::lazy_static;

use crate::math::Vec3;

use super::{
  defines::{parse_token_ok, ParserError},
  parser::{Loader, ParseLine},
  texture_loader::texture_loader,
};

#[derive(Debug, Default)]
struct MtlInfo {
  uid: u32,
  name: String,
  ambient: Vec3,
  diffuse: Vec3,
  specular: Vec3,
  emissive_coeficient: Vec3,
  specular_exponent: f32,
  dissolve: f32,
  transmission_filter: Vec3,
  optical_density: f32,
  receive_shadow: bool,
  illum: u8,
}

#[derive(Debug, Default)]
struct MtlData {
  mtls: Vec<MtlInfo>,
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

impl ParseLine<MtlData> for MtlParserImpl {
  fn parse_line(
    data: &mut MtlData,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    token_str: &str,
  ) -> super::defines::ParserResult {
    match token_str {
      "#" => {}

      "map_Ka" => {}
      _ => {}
    }

    Ok(())
    // let mut  path  = PathBuf::from("23");

    // path.push("")

    // path.to_str().ok_or(err)?.to_string();
  }
}

type MtlLoader = Loader<MtlData, MtlParserImpl>;

lazy_static! {
  pub static ref mtl_loader: MtlLoader = Default::default();
}
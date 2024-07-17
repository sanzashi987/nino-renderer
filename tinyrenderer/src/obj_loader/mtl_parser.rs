// mtl -> Material Template Library
use crate::math::Vec3;

use super::{
  defines::{self, parse_num, parse_token, parse_token_ok, ParserError},
  material::{Materials, Textures},
  parser::{ParseLine, Parser},
};

pub struct MtlParserImpl;

macro_rules! parse_texture_token {
  ($expr:expr;$type:ty) => {
  {
    let name parse_token_ok!($expr;%type);
    current.register_texture()
    name
  }
  };
}

impl ParseLine<Materials> for MtlParserImpl {
  fn parse_line(
    data: &mut Materials,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    s: &str,
  ) -> Result<(), ParserError> {
    let current = data.get_current()?;
    let map = &mut current.texture_map;

    match s {
      "#" => {}
      "newmtl" => data.new_material(&parse_token!(tokens.next();String)?),
      "Ns" => current.specular_exponent = parse_token_ok!(tokens.next();f32),
      "Ka" => current.ambient = parse_token_ok!(tokens.next();Vec3=x:f32,y:f32,z:f32),
      "Kd" => current.diffuse = parse_token_ok!(tokens.next();Vec3=x:f32,y:f32,z:f32),
      "Ks" => current.specular = parse_token_ok!(tokens.next();Vec3=x:f32,y:f32,z:f32),
      "Ke" => current.emissive_coeficient = parse_token_ok!(tokens.next();Vec3=x:f32,y:f32,z:f32),
      "Tf" => current.transmission_filter = parse_token_ok!(tokens.next();Vec3=x:f32,y:f32,z:f32),
      "Ni" => current.optical_density = parse_token_ok!(tokens.next();f32),
      "d" => current.dissolve = parse_token_ok!(tokens.next();f32),
      "Tr" => current.dissolve = parse_token_ok!(tokens.next();f32),
      "illum" => current.illum = parse_token_ok!(tokens.next();u8),
      "map_Ka" => map.ambient = parse_token_ok!(tokens.next();String),
      "map_Kd" => map.diffuse = parse_token_ok!(tokens.next();String),
      "map_Ks" => map.specular_color = parse_token_ok!(tokens.next();String),
      "map_Ns" => map.specular_highlight = parse_token_ok!(tokens.next();String),
      "map_d" => map.alpha = parse_token_ok!(tokens.next();String),
      "map_refl" => map.refl = parse_token_ok!(tokens.next();String),
      "map_Bump" => map.bump = parse_token_ok!(tokens.next();String),
      _ => {}
    }
    Ok(())
  }
}

type MtlParser<'a, 'b> = Parser<'a, 'b, Materials, MtlParserImpl>;

pub fn load_mtl<'a, 'b>(
  relative_path: &'a str,
  materials: &mut Materials,
) -> Result<MtlParser<'a, 'b>, ParserError>
where
  'a: 'b,
{
  let filepath = std::path::Path::new(relative_path);
  let mut parser = MtlParser::new(filepath)?;
  Ok(parser)
}

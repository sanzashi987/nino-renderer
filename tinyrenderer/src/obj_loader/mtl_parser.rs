// mtl -> Material Template Library
use crate::math::Vec3;

use super::{
  defines::{self, parse_num, parse_token, parse_token_ok, ParserError},
  material::{Materials, MoveTexutures, Textures},
  parser::{ParseLine, Parser},
};

pub struct MtlParserImpl;

impl ParseLine<Materials> for MtlParserImpl {
  fn parse_line(
    data: &mut Materials,
    tokens: &mut std::str::SplitWhitespace,
    s: &str,
  ) -> Result<(), ParserError> {
    // let texture_store = data
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

impl<'a, 'b> MoveTexutures for MtlParser<'a, 'b>
where
  'a: 'b,
{
  fn move_in_textures(&mut self, textures: Textures) {
    self.get_data_mut().move_in_textures(textures);
  }

  fn move_out_textures(self) -> Textures {
    self.get_data_own().move_out_textures()
  }
}

pub fn load_mtl(
  relative_path: &str,
  global_textures: Textures,
  mode: defines::ParserMode,
) -> Result<MtlParser, ParserError> {
  let filepath = std::path::Path::new(relative_path);
  let mut parser = MtlParser::new(filepath, mode)?;
  parser.move_in_textures(global_textures);
  Ok(parser)
}

// mtl -> Material Template Library
use crate::math::Vec3;

use super::{
  defines::{self, parse_num, parse_token, parse_token_ok, ParserError},
  material::{MoveMaterials, Mtl, MtlStores, Textures},
  parser::{ParseLine, Parser},
};

pub struct MtlParserImpl;

macro_rules! parse_texture_token {
  ($expr:expr; $textures:ident; $dir:ident) => {
    {
      let name = parse_token_ok!($expr;String);
      if let Some(n) = &name {
        let mut filepath = $dir.to_string();
        filepath.push_str(&n);

        $textures.load(&filepath, n);
      }
      name
    }
  };
}

impl ParseLine<Mtl> for MtlParserImpl {
  fn parse_line(
    data: &mut Mtl,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    s: &str,
  ) -> Result<(), ParserError> {
    let (current, texutures) = data.0.get_mutates()?;
    let map = &mut current.texture_map;

    match s {
      "#" => {}
      "newmtl" => data
        .0
        .materials
        .new_material(&parse_token!(tokens.next();String)?),
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
      "map_Ka" => map.ambient = parse_texture_token!(tokens.next();texutures;working_dir),
      "map_Kd" => map.diffuse = parse_texture_token!(tokens.next();texutures;working_dir),
      "map_Ks" => map.specular_color = parse_texture_token!(tokens.next();texutures;working_dir),
      "map_Ns" => {
        map.specular_highlight = parse_texture_token!(tokens.next();texutures;working_dir)
      }
      "map_d" => map.alpha = parse_texture_token!(tokens.next();texutures;working_dir),
      "map_refl" => map.refl = parse_texture_token!(tokens.next();texutures;working_dir),
      "map_Bump" => map.bump = parse_texture_token!(tokens.next();texutures;working_dir),
      "norm" => map.norm = parse_texture_token!(tokens.next();texutures;working_dir),
      _ => {}
    }
    Ok(())
  }
}

type MtlParser<'a, 'b> = Parser<'a, 'b, Mtl, MtlParserImpl>;

pub fn load_mtl<'a, 'b>(
  relative_path: &'a str,
  materials: MtlStores,
) -> Result<MtlParser<'a, 'b>, ParserError>
where
  'a: 'b,
{
  let filepath = std::path::Path::new(relative_path);
  let mut parser = MtlParser::new(filepath)?;
  parser.get_data().move_in_materials(materials);
  // parser.
  Ok(parser)
}

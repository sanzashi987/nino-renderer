use std::collections::HashMap;

use crate::math::Vec3;

use super::{
  error::Error,
  marcos::{parse_material_field, parse_num, parse_token, skip_to_next_line},
  token_requester::{TokenRequester, TokenType},
  Material, MtlLib,
};

pub struct MtlLibParser<'a> {
  requester: &'a mut TokenRequester<'a>,
}

impl<'a> MtlLibParser<'a> {
  pub fn new(requester: &'a mut TokenRequester<'a>) -> Self {
    Self { requester }
  }

  pub fn parse(&mut self) -> Result<MtlLib, Error> {
    let mut mtllib = MtlLib {
      materials: HashMap::new(),
    };

    let mut mtl: Option<Material> = None;

    let mut token = self.requester.request();

    let mut done = false;

    while !done {
      match token {
        TokenType::Token(token_str) => match token_str {
          "#" => {
            skip_to_next_line!(token = self.requester.request();TokenType::Eof,TokenType::Nextline)
          }
          "newmtl" => {
            if let Some(m) = mtl {
              // commit the last WIP material to the store
              mtllib.materials.insert(m.name.clone(), m);
            }
            mtl = Some(Material::new(
              &parse_token!(token = self.requester.request();String)?,
            ))
          }
          "Ns" => {
            parse_material_field!(
              mtl.specular_exponent = Some(parse_token!(token = self.requester.request();f32)?)
            )
          }
          "Ka" => {
            parse_material_field!(
              mtl.ambient =
                Some(parse_token!(token = self.requester.request();Vec3 = x:f32,y:f32,z:f32)?)
            )
          }
          "Kd" => {
            parse_material_field!(
              mtl.diffuse =
                Some(parse_token!(token = self.requester.request();Vec3 = x:f32,y:f32,z:f32)?)
            )
          }
          "Ks" => {
            parse_material_field!(
              mtl.specular =
                Some(parse_token!(token = self.requester.request();Vec3 = x:f32,y:f32,z:f32)?)
            )
          }
          "Ke" => {
            parse_material_field!(
              mtl.emissive_coeficient =
                Some(parse_token!(token = self.requester.request();Vec3 = x:f32,y:f32,z:f32)?)
            )
          }
          "Tf" => {
            parse_material_field!(
              mtl.transmission_filter =
                Some(parse_token!(token = self.requester.request(); Vec3=x:f32,y:f32,z:f32)?)
            )
          }
          "Ni" => {
            parse_material_field!(
              mtl.optical_density = Some(parse_token!(token = self.requester.request();f32)?)
            )
          }
          "d" => parse_material_field![
            mtl.dissolve = Some(parse_token![token = self.requester.request(); f32]?)
          ],
          "Tr" => parse_material_field![
            mtl.dissolve = Some(1.0 - parse_token![token = self.requester.request(); f32]?)
          ],
          "illum" => parse_material_field![
            mtl.illum = Some(parse_token![token = self.requester.request(); u8]?)
          ],
          "map_Ka" => parse_material_field![
            mtl.texture_maps.ambient =
              Some(parse_token![token = self.requester.request(); String]?)
          ],
          "map_Kd" => parse_material_field![
            mtl.texture_maps.diffuse =
              Some(parse_token![token = self.requester.request(); String]?)
          ],
          "map_Ks" => parse_material_field![
            mtl.texture_maps.specular_color =
              Some(parse_token![token = self.requester.request(); String]?)
          ],
          "map_Ns" => parse_material_field![
            mtl.texture_maps.specular_highlight =
              Some(parse_token![token = self.requester.request(); String]?)
          ],
          "map_d" => parse_material_field![
            mtl.texture_maps.alpha = Some(parse_token![token = self.requester.request(); String]?)
          ],
          "map_refl" => parse_material_field![
            mtl.texture_maps.refl = Some(parse_token![token = self.requester.request(); String]?)
          ],
          "map_Bump" => parse_material_field![
            mtl.texture_maps.bump = Some(parse_token![token = self.requester.request(); String]?)
          ],
          _ => return Err(Error::InvalidSyntax),
        },
        TokenType::Nextline => {
          token = self.requester.request();
        }
        TokenType::Eof => {
          if let Some(m) = mtl {
            mtllib.materials.insert(m.name.clone(), m);
            mtl = None;
          }
          done = true;
        }
      }
    }

    Ok(mtllib)
  }
}

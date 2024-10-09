use lazy_static::lazy_static;

use crate::textures::texture::Texture;

use super::{
  defines::ParserError,
  parser::{AssignId, Loader, Parse},
};

struct VoidParser {}

impl AssignId for Texture {}

impl Parse<Texture> for VoidParser {
  fn parse(full_path: &str, id: u32) -> Result<Texture, ParserError> {
    let res = Texture::load(full_path, id).map_err(|e| ParserError::TextureError(e))?;
    Ok(res)
  }
}

pub type TextureLoader = Loader<Texture, VoidParser>;

lazy_static! {
  pub static ref texture_loader: TextureLoader = Default::default();
}

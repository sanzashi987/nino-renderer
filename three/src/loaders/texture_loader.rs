use lazy_static::lazy_static;

use crate::textures::texture::Texture;

use super::{
  defines::ParserError,
  parser::{AssignId, Loader, Parse},
};

struct VoidParser {}

impl AssignId for Texture {}

impl Parse<Texture> for VoidParser {
  fn parse(path: &str, id: u32) -> Result<Texture, ParserError> {
    let res = Texture::load(path, id).map_err(|e| ParserError::TextureError(e))?;
    Ok(res)
  }

  fn parse_line(
    data: &mut Texture,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    token_str: &str,
  ) -> super::defines::ParserResult {
    todo!()
  }
}

pub type TextureLoader = Loader<Texture, VoidParser>;

lazy_static! {
  pub static ref texture_loader: TextureLoader = Default::default();
}

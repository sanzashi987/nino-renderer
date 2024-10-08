use lazy_static::lazy_static;

use crate::textures::texture::Texture;

use super::{defines::ParserError, parser::Loader};

struct VoidParser {}

pub type TextureLoader = Loader<Texture, VoidParser>;

impl TextureLoader {
  fn parse(&mut self, path: &str) -> Result<Texture, ParserError> {
    let res = Texture::load(path, self.next_id).map_err(|e| ParserError::TextureError(e))?;
  }
}

lazy_static! {
  pub static ref texture_loader: TextureLoader = Default::default();
}

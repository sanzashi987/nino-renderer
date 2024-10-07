use lazy_static::lazy_static;

use crate::textures::texture::Texture;

use super::{defines::ParserError, parser::Loader};

struct VoidParser {}

pub type TextureLoader = Loader<Texture, VoidParser>;

impl TextureLoader {
  pub fn load(&mut self, filepath: &str) -> Result<&Texture, ParserError> {
    if let Some(data) = self.if_exist(filepath) {
      return Ok(data);
    }
    let res = Texture::load(filepath, self.next_id).map_err(|e| ParserError::TextureError(e))?;

    let uid = self.insert_data(res, filepath);
    Ok(self.loaded.get(&uid).unwrap())
  }
}

lazy_static! {
  pub static ref texture_loader: TextureLoader = Default::default();
}

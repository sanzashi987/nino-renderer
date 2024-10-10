use lazy_static::lazy_static;

use crate::{textures::texture::Texture, utils::SingleOrList};

use super::{
  defines::ParserError,
  parser::{Loader, ILoaderData, Parse},
};

struct VoidParser {}

impl ILoaderData for Texture {
  fn get_name(&self) -> String {
    self.path.clone()
  }
}

impl Parse<Texture> for VoidParser {
  fn parse(full_path: &str, id: u32) -> Result<SingleOrList<Texture>, ParserError> {
    let res = Texture::load(full_path, id).map_err(|e| ParserError::TextureError(e))?;
    Ok(SingleOrList::Data(res))
  }
}

pub type TextureLoader = Loader<Texture, VoidParser>;

lazy_static! {
  pub static ref texture_loader: TextureLoader = Default::default();
}

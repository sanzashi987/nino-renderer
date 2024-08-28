use super::{buffer_attribute::TypeBufferEnum, marco::Extract};
use std::collections::HashMap;

pub struct BufferGeometry {
  attributes: Attribute,
}

pub type Attribute = HashMap<String, TypeBufferEnum>;

impl BufferGeometry {
  pub fn get_attribute(&self) -> &Attribute {
    let res = self.attributes.get("").map_or(None, |v| v.extract());
    &self.attributes
  }
}

macro_rules! attribute {
  ($attribute:ident,$type:ty,$key:tt) => {
    $attribute.get(k)
  };
}

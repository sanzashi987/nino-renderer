use super::{buffer_attribute::TypeBufferEnum, marco::Extract};
use std::collections::HashMap;

pub struct BufferGeometry {
  attributes: Attribute,
}

impl Default for BufferGeometry {
  fn default() -> Self {
    Self {
      attributes: Default::default(),
    }
  }
}

pub type Attribute = HashMap<String, TypeBufferEnum>;

impl BufferGeometry {
  pub fn get_attribute(&self) -> &Attribute {
    // let res = self.attributes.get("").map_or(None, |v| v.extract());
    &self.attributes
  }

  pub fn set_attribute(&mut self, key: &str, val: TypeBufferEnum) {
    self.attributes.insert(key.to_string(), val);
  }
}

macro_rules! attribute {
  ($attribute:ident,$type:ty,$key:tt) => {
    let res: Option<$type> = $attribute.get($key).map_or(None, |v| v.extract());
    res
  };

  ($attribute:ident, $type:ty, $key:tt, !) => {
    Extract::<$type>::extract(
      ($attribute
        .get($key)
        .expect(&format!("error from getting {} from attribute", $key))),
    )
    .expect(&format!(
      "errot from parsing attribute '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ))
  };
}

pub(crate) use attribute;

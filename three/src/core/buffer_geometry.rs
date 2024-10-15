use super::buffer_attribute::TypeBufferEnum;
use std::collections::HashMap;

pub struct BufferGeometry {
  attributes: Attribute,
  uuid: String,
}

impl Default for BufferGeometry {
  fn default() -> Self {
    Self {
      attributes: Default::default(),
      uuid: uuid::Uuid::new_v4().to_string(),
    }
  }
}

pub type Attribute = HashMap<String, TypeBufferEnum>;

pub trait IGeometry {
  fn get_uuid(&self) -> &str;
  fn get_attribute(&self) -> &Attribute;
  fn set_attribute(&mut self, key: &str, val: TypeBufferEnum);
}

impl IGeometry for BufferGeometry {
  fn get_attribute(&self) -> &Attribute {
    // let res = self.attributes.get("").map_or(None, |v| v.extract());
    &self.attributes
  }

  fn set_attribute(&mut self, key: &str, val: TypeBufferEnum) {
    self.attributes.insert(key.to_string(), val);
  }

  fn get_uuid(&self) -> &str {
    &self.uuid
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

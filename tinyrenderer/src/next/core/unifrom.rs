use std::collections::HashMap;

use crate::math::{Mat4, Vec2, Vec3, Vec4};

use super::marco::{define_gl_type_enum, Extract};

trait SetGlType<T> {
  fn set_attribute(&mut self, key: &str, val: T);
}

macro_rules! define_uniform_trait {
  ($name:tt; $enum_name:tt;$($prop:tt-$type:ty),+) => {
    define_gl_type_enum!($enum_name;$($prop-$type),+);
    $(
      impl SetGlType<$type> for $name {
        fn set_attribute(&mut self, key:&str ,val:$type){
          self.attributes.insert(key.to_string(),$enum_name::$prop(val));
        }
      }
    )+

  };
}

define_uniform_trait!(
  Uniform;
  UnifromTypeEnum;
  Int-i32,
  Float-f32,
  Vec2-Vec2,
  Vec3-Vec3,
  Vec4-Vec4,
  Mat4-Mat4
);

pub struct Uniform {
  attributes: HashMap<String, UnifromTypeEnum>,
}

impl Uniform {
  pub fn remove_attribute(&mut self, key: String) {
    self.attributes.remove(&key);
  }
}

macro_rules! u {
  ($store:ident, $type:ty, $key:tt,!) => {
    crate::next::core::Extract::<$type>::extract(
      ($store
        .get($key)
        .expect(&format!("error from getting {} from unifroms", $key))),
    )
    .except(&format!(
      "errot from parsing uniform '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ))
  };
  ($store:ident, $type:ty, $key:tt) => {{
    {
      let res: Option<$type> = $store.get($key).map_or(None, |v| v.extract());
      res
    }
  }};
}

pub(crate) use u;

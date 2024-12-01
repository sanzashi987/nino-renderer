use std::{
  collections::HashMap,
  ops::{Deref, DerefMut},
};

use crate::math::{Mat3, Mat4, Vec2, Vec3, Vec4};

use super::marco::define_gl_type_enum;

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
  UniformTypeEnum;
  Int-i32,
  Float-f32,
  Vec2-Vec2,
  Vec3-Vec3,
  Vec4-Vec4,
  Mat3-Mat3,
  Mat4-Mat4,
  Bool-bool,
  Uv-u32 // uv's uid
);
#[derive(Debug, Default)]
pub struct Uniform {
  attributes: HashMap<String, UniformTypeEnum>,
}

impl Deref for Uniform {
  type Target = HashMap<String, UniformTypeEnum>;

  fn deref(&self) -> &Self::Target {
    &self.attributes
  }
}

impl Uniform {
  pub fn get(&self, key: &str) -> Option<UniformTypeEnum> {
    let res = self.attributes.get(key);
    res.map(|x| *x)
  }

  pub fn insert<T: Into<UniformTypeEnum>>(&mut self, key: &str, val: T) {
    let typed_enum: UniformTypeEnum = val.into();
    self.attributes.insert(key.to_string(), typed_enum);
  }

  pub fn merge(&mut self, another: &Self) {
    another.iter().for_each(|(k, v)| {
      if self.attributes.contains_key(k) {
        return;
      }

      self.insert(k, *v);
    });
  }

  pub fn extends(mut self, mut from: Self) -> Self {
    for (k, v) in from.attributes.drain() {
      if self.attributes.contains_key(&k) {
        continue;
      }
      self.attributes.insert(k, v);
    }

    self
  }
}

macro_rules! u {
  ($store:ident, $type:ty, $key:tt,!) => {
    crate::core::Extract::<$type>::extract(
      ($store
        .get($key)
        .expect(&format!("error from getting {} from uniforms", $key))),
    )
    .expect(&format!(
      "error from parsing uniform '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ))
  };
  ($store:ident, $type:ty, $key:tt) => {{
    let res: Option<$type> = $store.get($key).map_or(None, |v| v.extract());
    res
  }};
}

pub(crate) use u;

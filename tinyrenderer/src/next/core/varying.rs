use std::{
  collections::HashMap,
  ops::{Add, Mul},
};

use crate::math::{Barycentric, Mat4, Vec2, Vec3, Vec4};

trait DeclareGlType<T> {
  fn declare_attribute(&mut self, key: &str, val: T);
}

macro_rules! define_varying_trait {
  ($name:tt; $enum_name:tt;$($prop:tt-$type:ty),+) => {
    crate::next::core::marco::define_gl_type_enum!($enum_name;$($prop-$type),+);
    $(

      impl DeclareGlType<$type> for $name {
        fn declare_attribute(&mut self, key:&str, val:$type){
          if !self.declare.contains_key(key) {
            self.declare.insert(key.to_string(), vec![]);
          }

          let vec = self.declare.get_mut(key).unwrap();
          vec.push($enum_name::$prop(val));
        }
      }

    )+


    impl Add<Self> for $enum_name {
      type Output = Self;
      fn add(self, rhs: Self) -> Self::Output {
        match self{
          $(
            Self::$prop(val) => {
              if let Self::$prop(r_val) = rhs {
                Self::$prop(val + r_val)
              } else {
                panic!()
              }
            }
          )+
        }
      }
    }
  };
}

define_varying_trait!(
  Varying;
  VaryingTypeEnum;
  Int-i32,
  Float-f32,
  Vec2-Vec2,
  Vec3-Vec3,
  Vec4-Vec4,
  Mat4-Mat4
);

impl Mul<f32> for VaryingTypeEnum {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    match self {
      VaryingTypeEnum::Int(val) => VaryingTypeEnum::Float(rhs * (val as f32)),
      VaryingTypeEnum::Float(val) => VaryingTypeEnum::Float(rhs * val),
      VaryingTypeEnum::Vec2(val) => VaryingTypeEnum::Vec2(val * rhs),
      VaryingTypeEnum::Vec3(val) => VaryingTypeEnum::Vec3(val * rhs),
      VaryingTypeEnum::Vec4(val) => VaryingTypeEnum::Vec4(val * rhs),
      VaryingTypeEnum::Mat4(val) => VaryingTypeEnum::Mat4(val * rhs),
    }
  }
}
#[derive(Debug, Default)]
pub struct Varying {
  declare: HashMap<String, Vec<VaryingTypeEnum>>,
  result: HashMap<String, VaryingTypeEnum>,
}

impl Varying {
  pub fn lerp(&mut self, bary: &Barycentric, rhws: [f32; 3], z: f32) {
    for key in self.declare.keys() {
      let vec = self.declare.get(key).unwrap();
      let length = vec.len();

      match length {
        1 => {
          let val = vec[0];
          self.result.insert(key.to_string(), val);
        }
        3 => {
          let arr = [vec[0] * rhws[0], vec[1] * rhws[1], vec[2] * rhws[2]];
          let lerped_val = bary.apply_weight(&arr) * z;
          self.result.insert(key.to_string(), lerped_val);
        }
        _ => continue,
      }
    }
  }
  pub fn get(&self, key: &str) -> Option<VaryingTypeEnum> {
    self.result.get(key).map(|x| *x)
  }
}

macro_rules! v {
  ($store:ident,$type:ty,$key:tt,!) => {
    crate::next::core::Extract::<$type>::extract(
      ($store
        .get($key)
        .expect(&format!("error from getting {} from varyings", $key))),
    )
    .expect(&format!(
      "error from parsing varying '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ))
  };
  ($store:ident,$type:ty,$key:tt) => {{
    let res: Option<$type> = $store.get($key).map_or(None, |v| v.extract());
    res
  }};
}

macro_rules! add_v {
  ($store:ident,$key:tt,$expr:expr) => {{
    let val = $expr;
    $store.declare_attribute($key, val);
  }};
  ($store:ident,$key:tt,$val:tt) => {
    $store.declare_attribute($key, $val);
  };
}

pub(crate) use add_v;
pub(crate) use v;

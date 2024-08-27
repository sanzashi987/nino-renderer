use std::{
  collections::HashMap,
  ops::{Add, Mul},
};

use crate::math::{Mat4, Vec2, Vec3, Vec4};

use super::marco::{define_gl_type_enum, Extract};

trait DeclareGlType<T> {
  fn declare_attribute(&mut self, key: &str, val: T);
}

macro_rules! define_varying_trait {
  ($name:tt; $enum_name:tt;$($prop:tt-$type:ty),+) => {
    define_gl_type_enum!($enum_name;$($prop-$type),+);
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

  
}

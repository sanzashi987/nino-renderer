use std::collections::HashMap;

struct TypeBufferAttribute<T: Sized> {
  data: Vec<T>,
  size: u32,
  normalized: bool,
}

impl<T: Sized> TypeBufferAttribute<T> {
  pub fn new(data: Vec<T>, size: u32, normalized: bool) -> Self {
    Self {
      data,
      size,
      normalized,
    }
  }
}

macro_rules! typed_array {
  ($($enum:tt-$type:tt-$ty:tt);+) => {
    pub enum TypeBufferEnum {
      $(
       $enum(Box<$type>),
      ) +
    }
    $(
      pub type $type  = TypeBufferAttribute<$ty>;
    )+
  };
}

typed_array!(
  F64-F64BufferAttribute-f64;
  F32-F32BufferAttribute-f32;
  U32-U32BufferAttribute-u32;
  I32-I32BufferAttribute-i32;
  U16-U16BufferAttribute-u16;
  I16-I16BufferAttribute-i16;
  U8-U8BufferAttribute-u8;
  I8-I8BufferAttribute-i8
);

pub struct Geometry {
  attributes: HashMap<String, TypeBufferEnum>,
}

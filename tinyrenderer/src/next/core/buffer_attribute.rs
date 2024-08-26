struct TypeBufferAttribute<T: Sized> {
  data: Vec<T>,
  size: usize,
  normalized: bool,
}

impl<T: Sized + Copy> TypeBufferAttribute<T> {
  pub fn new(data: Vec<T>, size: usize, normalized: bool) -> Self {
    Self {
      data,
      size,
      normalized,
    }
  }
}

impl<T: Sized + Copy> Iterator for TypeBufferAttribute<T> {
  type Item = Vec<T>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut res = vec![];
    let mut count = self.size;
    while count > 0 {
      if let Some(v) = self.data.iter().next() {
        res.push(*v);
      } else {
        return None;
      };

      count -= 1;
    }
    Some(res)
  }
}

macro_rules! typed_array {
  ($($enum:tt-$type:tt-$ty:tt);+) => {
    $(
      pub type $type = TypeBufferAttribute<$ty>;

      impl $type {
        fn as_enum(self)-> TypeBufferEnum{
          TypeBufferEnum::$enum(Box::new(self))
        }

      }
    )+
    pub enum TypeBufferEnum {
      $(
       $enum(Box<$type>),
      ) +
    }
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

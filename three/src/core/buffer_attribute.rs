use crate::math::Vec3;
pub struct TypeBufferAttribute<T: Sized + Copy + ToF32> {
  pub data: Vec<T>,
  pub size: usize,
  pub normalized: bool,
}

pub trait ToF32 {
  fn to(self) -> f32;
}

impl<T: Sized + Copy + ToF32> TypeBufferAttribute<T> {
  pub fn new(data: Vec<T>, size: usize, normalized: bool) -> Self {
    Self {
      data,
      size,
      normalized,
    }
  }

  pub fn pick(&self, index: usize) -> TypeBufferAttribute<T> {
    let start = self.size * index;
    let end = start + self.size + 1;

    let mut data = vec![];
    for i in start..end {
      data.push(self.data[i]);
    }
    Self {
      data,
      size: self.size,
      normalized: self.normalized,
    }
  }
}

pub trait IBufferAttribute<T: Sized + Copy + ToF32> {
  fn get_x(&self, index: usize) -> T;
  fn get_y(&self, index: usize) -> T;
  fn get_z(&self, index: usize) -> T;
  fn get_w(&self, index: usize) -> T;
  fn get_vec3(&self, index: usize) -> Vec3;
  fn items(&self) -> usize;
}

impl<T: Sized + Copy + ToF32> IBufferAttribute<T> for TypeBufferAttribute<T> {
  fn get_x(&self, index: usize) -> T {
    self.data[index * self.size]
  }

  fn get_y(&self, index: usize) -> T {
    self.data[index * self.size + 1]
  }

  fn get_z(&self, index: usize) -> T {
    self.data[index * self.size + 2]
  }

  fn get_w(&self, index: usize) -> T {
    self.data[index * self.size + 3]
  }

  fn items(&self) -> usize {
    self.data.len() / self.size
  }
  fn get_vec3(&self, index: usize) -> Vec3 {
    let x = self.get_x(index).to();
    let y = self.get_y(index).to();
    let z = self.get_z(index).to();
    Vec3::new(x, y, z)
  }
}

// impl Into<Vec3> for Vec<f32> {
//   fn into(self) -> Vec3 {
//     Vec3::new(self[0], self[1], self[2])
//   }
// }

pub trait ExtractRef<T> {
  fn extract(&self) -> Option<T>;
}

impl<T: Sized + Copy + ToF32> Iterator for TypeBufferAttribute<T> {
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
  ($enum_name:tt;$($enum:tt-$type:tt-$ty:tt);+) => {
    $(
      pub type $type = TypeBufferAttribute<$ty>;

      impl ToF32 for $ty {
        fn to(self)->f32{
          self as f32
        }
      }

      impl $type {
        pub fn as_enum(self)-> $enum_name{
          $enum_name::$enum(Box::new(self))
        }

      }


      // impl Extract<Box<$type>> for $enum_name {
      //   fn extract(self)->Option<Box<$type>>{
      //     if let Self::$enum(val) = self {
      //       Some(val)
      //     } else {
      //       None
      //     }
      //   }
      // }
    )+


    impl ExtractRef<Vec3> for  $enum_name {
      fn extract(&self) -> Option<Vec3> {
        let (x,y,z) = match self {
          $(
            Self::$enum(val)=> {
              if val.data.len() < 3{
                return None
              }
              (val.data[0] as f32,val.data[1] as f32,val.data[2] as f32)
            }
          )+
          _ => {
            return None
          }
        };
        Some(Vec3::new(x,y,z))
      }
    }
    pub enum $enum_name {
      $(
       $enum(Box<$type>),
      ) +
    }

    impl $enum_name {
      pub fn pick(&self, index:usize) -> $enum_name {
        match &self {
          $(
            Self::$enum(buffer)=>{
              return Self::$enum(Box::new(buffer.pick(index)));
            }
          )+
        }
      }
    }

  };
}

typed_array!(
  TypeBufferEnum;
  F64-F64BufferAttribute-f64;
  F32-F32BufferAttribute-f32;
  U32-U32BufferAttribute-u32;
  I32-I32BufferAttribute-i32;
  U16-U16BufferAttribute-u16;
  I16-I16BufferAttribute-i16;
  U8-U8BufferAttribute-u8;
  I8-I8BufferAttribute-i8
);

macro_rules! a {
  ($store:ident, $type:ty, $key:tt,!) => {
    crate::core::ExtractRef::<$type>::extract(
      ($store
        .get($key)
        .expect(&format!("error from getting {} from attributes", $key))),
    )
    .expect(&format!(
      "errot from parsing attribute '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ))
  };
}

pub(crate) use a;

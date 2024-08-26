macro_rules! define_gl_type_enum {
  ($enum_name:tt;$($name:tt-$type:ty),+) => {
    #[derive(Debug, Clone, Copy)]
    pub enum $enum_name {
      $(
        $name($type),
      )+
    }
    $(
      impl Extract<$type> for $enum_name {
        fn extract(self)->Option<$type>{
          if let Self::$name(val) = self {
            Some(val)
          } else {
            None
          }
        }
      }

      impl From<$type> for $enum_name{
        fn from(item:$type)->Self{
          Self::$name(item)
        }
      }

    )+
  };
}

pub(super) use define_gl_type_enum;

macro_rules! define_interpolatable_gl_type {
  ($enum_name:tt;$($name:tt-$type:ty),+) => {};
}

pub(super) use define_interpolatable_gl_type;

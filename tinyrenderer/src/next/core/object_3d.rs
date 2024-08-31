pub enum ObjectType {
  Light,
  Mesh,
  Scene,
  Object3D,
}

pub trait Object3DMethod<T> {
  fn add(&mut self, object: Box<T>);
}

pub struct Object3D<T> {
  object_type: ObjectType,
  parent: Option<String>,
  children: Vec<T>,
}

impl<T> Object3D<T> {
  pub fn new(object_type: ObjectType, parent: Option<String>, children: Vec<T>) -> Self {
    Self {
      object_type,
      parent,
      children,
    }
  }
  pub fn set_parent(&mut self, parent: String) {
    self.parent = Some(parent);
  }

  pub fn get_parent(&self) -> Option<String> {
    self.parent.clone()
  }

  pub fn add(&mut self, obj: T) {
    self.children.push(obj)
  }
}

// impl<T> Object3DMethod<T> for Object3D<T> {
//   fn add(&mut self, obj: Box<T>) {
//     self.children.push(obj)
//   }
// }

macro_rules! define_objects {
  ($enum_name:tt;$($name:tt:$ty:ty),+) => {
    pub enum $enum_name {
      $(
        $name($ty),
      )+
    }
    impl $enum_name {

      const supported_type: &'static[&'static str] = [
      $(std::any::type_name::<$ty>()),+
      ];

      pub fn convert(val :T) ->Option<Self>{
        let input_type_name = std::any::type_name::<T>();
        let mut i = 0;
        $(
          if Self::supported_type[i] == input_type_name {
            return Some(Self::$name(val))
          } else {
            i+=1;
          }
        )+



      }
    }


    impl From<$type> for $enum_name{
      fn from(item:$ty)->Self{
        Self::$name(item)
      }
    }
  };
}

pub enum ObjectType {
  Light,
  Mesh,
  Scene,
  Object3D,
}

pub trait Object3DMethod {
  fn add<T: 'static + Sized>(&mut self, object: T) -> bool;
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

macro_rules! define_support_objects {
  ($enum_name:tt;$($name:tt:$ty:ty),+) => {
    pub enum $enum_name {
      $(
        $name($ty),
      )+
    }
    impl $enum_name {
      #[allow(unused)]
      pub fn convert<T:'static + Sized>(val :T) ->Option<Self>{
        let val_any: Box<dyn std::any::Any> = Box::new(val);
        $(
          let val_any = match val_any.downcast::<$ty>() {
            Ok(matched) =>{
              return Some(Self::$name(*matched));
            },
            Err(instance) =>{
              instance
            }
          };
        )+

        return None;

      }
    }

  };
}

pub(crate) use define_support_objects;

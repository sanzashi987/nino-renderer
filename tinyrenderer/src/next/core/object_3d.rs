use crate::math::{Mat4, Vec3};
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

pub enum ObjectType {
  Light,
  Mesh,
  Scene,
  Object3D,
  Camera,
}

impl Default for ObjectType {
  fn default() -> Self {
    Self::Object3D
  }
}

pub trait ObjectActions {
  fn transform_matrix(&self) -> &crate::math::Mat4;
  fn set_parent(&self, parent: Rc<dyn ObjectActions>);
  fn get_parent(&self) -> Option<Rc<dyn ObjectActions>>;
  fn add<T: 'static + Sized>(&self, val: T);
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

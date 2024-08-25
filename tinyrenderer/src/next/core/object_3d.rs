pub enum ObjectType {
  Light,
  Mesh,
}

pub trait IObject3D {
  fn add(&mut self, object: Box<dyn IObject3D>);
}

macro_rules! Object3D {
  ($name:tt; $type:tt;) => {
    pub struct $name {
      pub object_type: ObjectType,
      pub parent: Option<String>,
      pub children: Vec<Box<dyn IObject3D>>,
    }
    impl $name {
      pub fn new(object_type:ObjectType) ->Self{
        Self {
          object_type,
          parent:None,
          children:vec![]
        }
      }

    }

    impl IObject3D for $name {
      fn add(&mut self, obj ) {
        self.children.push(obj)
      }
    }
  };
}

pub(crate) use Object3D;

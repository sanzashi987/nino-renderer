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

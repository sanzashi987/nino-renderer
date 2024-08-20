use super::object_3d::Object3D;

pub struct Scene {
  children: Vec<Box<dyn Object3D>>,
}

impl Scene {
  pub fn add(&mut self, obj: Box<dyn Object3D>) {
    self.children.push(obj)
  }
}

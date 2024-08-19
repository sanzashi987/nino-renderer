use super::object_3d::{Object3D, ObjectType};

pub struct DirectionalLight {}

impl DirectionalLight {
  pub fn new() -> Self {
    Self {}
  }
}

impl Object3D for DirectionalLight {
  fn get_type(&self) -> ObjectType {
    ObjectType::Light
  }
}

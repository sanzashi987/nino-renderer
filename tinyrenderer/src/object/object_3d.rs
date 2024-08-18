pub enum ObjectType {
  Object3D,
  Light,
  Mesh,
}

pub trait Object3D {
  fn get_type() -> ObjectType;
}

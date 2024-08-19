use super::object_3d::Object3D;

pub struct Scene {
  children: Vec<Box<dyn Object3D>>,
}

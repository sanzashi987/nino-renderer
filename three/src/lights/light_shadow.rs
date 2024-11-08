use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  math::{Mat4, Vec2},
};

pub struct LightShadow {
  camera: Rc<dyn ICamera>,
  intensity: i32,
  bias: i32,
  normal_bias: i32,
  radius: i32,
  // shadow texture width & height
  map_size: Vec2,
  mat: Mat4,
}
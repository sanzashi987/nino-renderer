use crate::math::Vec3;

use super::buffer_attribute::{IBufferAttribute, ToF32};

pub struct Box {
  min: Vec3,
  max: Vec3,
}

impl Box {
  fn expand(&mut self, point: Vec3) {
    self.min.min(point);
    self.max.max(point);
  }

  fn get_center(&self) -> Vec3 {
    let full = self.min + self.max;
    full / 2.0
  }

  fn reset(&mut self) {
    self.min.x = f32::INFINITY;
    self.min.y = f32::INFINITY;
    self.min.z = f32::INFINITY;
    self.max.x = -f32::INFINITY;
    self.max.y = -f32::INFINITY;
    self.max.z = -f32::INFINITY;
  }

  fn from_attribute<T: Sized + Copy + ToF32>(&mut self, attribute: &impl IBufferAttribute<T>) {
    self.reset();

    for index in 0..attribute.count() {
      let x: f32 = attribute.get_x(index).to();
      let y: f32 = attribute.get_y(index).to();
      let z: f32 = attribute.get_z(index).to();
      let point = Vec3::new(x, y, z);
      self.expand(point);
    }
  }
}

pub struct Sphere {
  center: Vec3,
  radius: i32,
}

impl Sphere {}

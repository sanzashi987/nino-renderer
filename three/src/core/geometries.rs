use crate::math::Vec3;

use super::buffer_attribute::{IBufferAttribute, ToF32};

#[derive(Debug, Default)]
pub struct Box3 {
  min: Vec3,
  max: Vec3,
}

impl Box3 {
  pub fn expand(&mut self, point: Vec3) {
    self.min.min(point);
    self.max.max(point);
  }

  pub fn get_center(&self) -> Vec3 {
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

  // pub fn from_attribute<T: Sized + Copy + ToF32>(&mut self, attribute: &impl IBufferAttribute<T>) {
  pub fn from_attribute<T: Sized + Copy + ToF32>(
    &mut self,
    attribute: &Box<dyn IBufferAttribute<T>>,
  ) {
    self.reset();

    for index in 0..attribute.items() {
      let x: f32 = attribute.get_x(index).to();
      let y: f32 = attribute.get_y(index).to();
      let z: f32 = attribute.get_z(index).to();
      let point = Vec3::new(x, y, z);
      self.expand(point);
    }
  }
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Default for Sphere {
  fn default() -> Self {
    Self {
      center: Vec3::zero(),
      radius: f32::INFINITY,
    }
  }
}

pub trait IBoundingSphere {
  fn update_bounding_sphere(&mut self);
  fn bounding_sphere(&self) -> &Sphere;
}

impl Sphere {}

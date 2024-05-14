use crate::math::{Vec2, Vec3, Vec4};

const ATTR_NUM: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct Attributes {
  pub float: [f32; ATTR_NUM],
  pub vec2: [Vec2; ATTR_NUM],
  pub vec3: [Vec3; ATTR_NUM],
  pub vec4: [Vec4; ATTR_NUM],
}

impl Attributes {
  pub fn set_float(&mut self, location: usize, value: f32) {
    self.float[location] = value;
  }
  pub fn set_vec2(&mut self, location: usize, value: Vec2) {
    self.vec2[location] = value;
  }
  pub fn set_vec3(&mut self, location: usize, value: Vec3) {
    self.vec3[location] = value;
  }
  pub fn set_vec4(&mut self, location: usize, value: Vec4) {
    self.vec4[location] = value;
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
  pub position: Vec4,
  pub attributes: Attributes,
}

impl Vertex {
  pub fn new(position: &Vec3, attributes: Attributes) -> Self {
    Self {
      position: Vec4::from_vec3(position, 1.0),
      attributes,
    }
  }

  pub fn truncated_to_vec2(&self) -> Vec2 {
    self.position.truncated_to_vec2()
  }
}

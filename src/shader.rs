use std::{collections::HashMap, default};

use crate::{
  math::{lerp, Mat4, Vec2, Vec3, Vec4},
  texture::TextureStore,
};

const ATTR_NUM: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct Attributes {
  pub float: [f32; ATTR_NUM],
  pub vec2: [Vec2; ATTR_NUM],
  pub vec3: [Vec3; ATTR_NUM],
  pub vec4: [Vec4; ATTR_NUM],
}

impl Default for Attributes {
  fn default() -> Self {
    Self {
      float: [0.0; ATTR_NUM],
      vec2: [Vec2::zero(); ATTR_NUM],
      vec3: [Vec3::zero(); ATTR_NUM],
      vec4: [Vec4::zero(); ATTR_NUM],
    }
  }
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

// Runtime Vertex in pipeline
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

// perspective correction in advance
pub fn vertex_rhw_init(vertex: &mut Vertex) {
  // let rhw_z = 1.0 / vertex.position.z;
  vertex.position.z = 1.0 / vertex.position.z;
  // apply perspective correction
  attributes_foreach(&mut vertex.attributes, |v| v * vertex.position.z);
}

pub fn lerp_vertex(start: &Vertex, end: &Vertex, t: f32) -> Vertex {
  let position = start.position + (end.position - start.position) * t;
  let attributes = interp_attributes(&start.attributes, &end.attributes, lerp, t);

  Vertex {
    position,
    attributes,
  }
}

pub fn interp_attributes<F>(attr1: &Attributes, attr2: &Attributes, f: F, t: f32) -> Attributes
where
  F: Fn(f32, f32, f32) -> f32,
{
  let mut attributes = Attributes::default();

  for index in 0..ATTR_NUM {
    attributes.set_float(index, f(attr1.float[index], attr2.float[index], t));
  }

  for index in 0..ATTR_NUM {
    let value1 = attr1.vec2[index];
    let value2 = attr2.vec2[index];
    attributes.set_vec2(
      index,
      Vec2::new(f(value1.x, value2.x, t), f(value1.y, value2.y, t)),
    );
  }

  for index in 0..ATTR_NUM {
    let value1 = attr1.vec3[index];
    let value2 = attr2.vec3[index];
    attributes.set_vec3(
      index,
      Vec3::new(
        f(value1.x, value2.x, t),
        f(value1.y, value2.y, t),
        f(value1.z, value2.z, t),
      ),
    );
  }

  for index in 0..ATTR_NUM {
    let value1 = attr1.vec4[index];
    let value2 = attr2.vec4[index];
    attributes.set_vec4(
      index,
      Vec4::new(
        f(value1.x, value2.x, t),
        f(value1.y, value2.y, t),
        f(value1.z, value2.z, t),
        f(value1.w, value2.w, t),
      ),
    );
  }

  attributes
}

pub fn attributes_foreach<F>(attr: &mut Attributes, f: F)
where
  F: Fn(f32) -> f32,
{
  for index in 0..ATTR_NUM {
    attr.set_float(index, f(attr.float[index]));
  }

  for index in 0..ATTR_NUM {
    let value = attr.vec2[index];
    attr.set_vec2(index, Vec2::new(f(value.x), f(value.y)));
  }

  for index in 0..ATTR_NUM {
    let value = attr.vec3[index];
    attr.set_vec3(index, Vec3::new(f(value.x), f(value.y), f(value.z)));
  }

  for index in 0..ATTR_NUM {
    let value = attr.vec4[index];
    attr.set_vec4(
      index,
      Vec4::new(f(value.x), f(value.y), f(value.z), f(value.w)),
    );
  }
}

#[derive(Default)]
pub struct Uniforms {
  pub int: HashMap<u32, i32>,
  pub float: HashMap<u32, f32>,
  pub vec2: HashMap<u32, Vec2>,
  pub vec3: HashMap<u32, Vec3>,
  pub vec4: HashMap<u32, Vec4>,
  pub mat4: HashMap<u32, Mat4>,
  pub texture: HashMap<u32, u32>,
}

impl Uniforms {
  pub fn clear(&mut self) {
    self.int.clear();
    self.float.clear();
    self.vec2.clear();
    self.vec3.clear();
    self.vec4.clear();
    self.mat4.clear();
    self.texture.clear();
  }
}

pub type VertexShading = Box<dyn Fn(&Vertex, &Uniforms, &TextureStore) -> Vertex>;
pub type FragmentShading = Box<dyn Fn(&Attributes, &Uniforms, &TextureStore) -> Vec4>;

pub struct Shader {
  pub vertex_shading: VertexShading,
  pub fragment_shading: FragmentShading,
  pub uniforms: Uniforms,
}

impl Shader {
  pub fn call_vertex_shading(&self, v: &Vertex, u: &Uniforms, s: &TextureStore) -> Vertex {
    (self.vertex_shading)(v, u, s)
  }
  pub fn call_fragment_shading(&self, a: &Attributes, u: &Uniforms, s: &TextureStore) -> Vec4 {
    (self.fragment_shading)(a, u, s)
  }
}

impl Default for Shader {
  fn default() -> Self {
    Self {
      vertex_shading: Box::new(|x, _, _| *x),
      fragment_shading: Box::new(|_, _, _| Vec4::new(0.0, 0.0, 0.0, 1.0)),
      uniforms: Default::default(),
    }
  }
}

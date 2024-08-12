use std::{
  collections::HashMap,
  fmt::Debug,
  ops::{Add, Mul},
};

use crate::{
  math::{Barycentric, Mat4, Vec2, Vec3, Vec4},
  model::Vertex,
};

use super::material::Textures;

pub trait Extract<T> {
  fn extract(self) -> Option<T>;
}

macro_rules! uniform {
  ($store:ident, $type:ty, $key:tt, !) => {
    Extract::<$type>::extract(
      ($store
        .get($key)
        .expect(&format!("error from getting {} from unifroms", $key))),
    )
    .expect(&format!(
      "errot from parsing uniform '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ));
  };
  ($store:ident, $type:ty, $key:tt) => {{
    {
      let res: Option<$type> = $store.get($key).map_or(None, |v| v.extract());
      res
    }
  }};
}

macro_rules! varying {
  ($store:ident, $type:ty, $key:tt, !) => {
    uniform!($store, $type, $key, !)
  };
  ($store:ident, $type:ty, $key:tt) => {
    uniform!($store, $type, $key)
  };
}
pub(crate) use uniform;
pub(crate) use varying;

macro_rules! define_union_type_enum {
  ($enum:tt;$($name:tt@$type:ty),+) => {
    #[derive(Debug, Clone, Copy)]
    pub enum $enum {
      $(
        $name($type),
      )+
    }

    impl Add<Self> for $enum {
      type Output = Self;
      fn add(self, rhs: Self) -> Self::Output {
        match self{
          $(
            Self::$name(val) => {
              if let Self::$name(r_val) = rhs {
                Self::$name(val + r_val)
              } else {
                panic!()
              }
            }
          )+
        }
      }
    }

    $(
      impl Extract<$type> for $enum {
        fn extract(self)->Option<$type>{
          if let Self::$name(val) = self {
            Some(val)
          } else {
            None
          }
        }
      }
    )+
  };
}

define_union_type_enum!(
  GLTypes;
  Int@i32,
  Float@f32,
  Vec2@Vec2,
  Vec3@Vec3,
  Vec4@Vec4,
  Mat4@Mat4
);

impl Mul<f32> for GLTypes {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    match self {
      GLTypes::Int(val) => GLTypes::Float(rhs * (val as f32)),
      GLTypes::Float(val) => GLTypes::Float(rhs * val),
      GLTypes::Vec2(val) => GLTypes::Vec2(val * rhs),
      GLTypes::Vec3(val) => GLTypes::Vec3(val * rhs),
      GLTypes::Vec4(val) => GLTypes::Vec4(val * rhs),
      GLTypes::Mat4(val) => GLTypes::Mat4(val * rhs),
    }
  }
}
#[derive(Debug, Default)]
pub struct Varyings {
  data: HashMap<String, Vec<GLTypes>>,
}

impl Varyings {
  pub fn new() -> Self {
    Self {
      data: HashMap::new(),
    }
  }
  pub fn set(&mut self, key: &str, gl_values: GLTypes) {
    if !self.data.contains_key(key) {
      self.data.insert(key.to_string(), vec![]);
    }

    let val = self.data.get_mut(key).unwrap();
    val.push(gl_values);

    // self.data.insert(key.to_string(), val);
  }
}

pub type GlTypeMap = HashMap<String, GLTypes>;

#[derive(Debug, Default)]
pub struct Varying {
  data: GlTypeMap,
}

impl Varying {
  pub fn set(&mut self, key: &str, gl_values: GLTypes) {
    self.data.insert(key.to_string(), gl_values);
  }

  pub fn get(&self, key: &str) -> Option<GLTypes> {
    self.data.get(key).map(take_value)
  }
}

pub fn take_value<T: Copy>(v: &T) -> T {
  *v
}

pub struct Uniform<'a> {
  global: &'a GlTypeMap,
  data: GlTypeMap,
}

impl<'a> Uniform<'a> {
  pub fn new(global: &'a GlTypeMap, data: GlTypeMap) -> Self {
    Self { global, data }
  }

  pub fn get(&self, key: &str) -> Option<GLTypes> {
    let res = self.data.get(key);

    if res.is_none() {
      self.global.get(key).map(take_value)
    } else {
      res.map(take_value)
    }
  }

  pub fn set(&mut self, key: &str, gl_values: GLTypes) {
    self.data.insert(key.to_string(), gl_values);
  }
}

type VertexShader = Box<dyn Fn(&Vertex, &Uniform, &mut Varyings) -> Vertex>;
type FragmentShader = Box<dyn Fn(&Uniform, &Varying, &Textures) -> Vec4>;

pub struct Shader {
  pub vertex: VertexShader,
  pub fragment: FragmentShader,
}

impl Debug for Shader {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Shader")
      .field("vertex", &"/** vertex clousure */".to_string())
      .field("fragment", &"/** fragment clousure */".to_string())
      .finish()
  }
}
impl Default for Shader {
  fn default() -> Self {
    Self {
      vertex: Self::default_vertex(),
      fragment: Self::default_fragment(),
    }
  }
}

impl Shader {
  pub fn default_vertex() -> VertexShader {
    let vertex: VertexShader = Box::new(|v, u, _| {
      let model_matrix = uniform!(u, Mat4, "model_matrix", !);
      let view_matrix = uniform!(u, Mat4, "view_matrix", !);
      let projection_matrix = uniform!(u, Mat4, "projection_matrix", !);
      let mut next_v = *v;
      next_v.position = projection_matrix * view_matrix * model_matrix * next_v.position;

      next_v
    });
    vertex
  }
  pub fn default_fragment() -> FragmentShader {
    let fragment: FragmentShader = Box::new(|_, _, _| Vec4::new(1.0, 1.0, 1.0, 1.0));
    fragment
  }

  pub fn run_vertex(
    &self,
    gl_vertex: &Vertex,
    uniforms: &Uniform,
    varyings: &mut Varyings,
  ) -> Vertex {
    (self.vertex)(gl_vertex, uniforms, varyings)
  }

  pub fn run_fragment(
    &self,
    bar: &Barycentric,
    uniforms: &Uniform,
    varyings: &Varyings,
    textures: &Textures,
    rhws: [f32; 3],
    z: f32,
  ) -> Vec4 {
    let varying = self.lerp_varyings(bar, varyings, rhws, z);

    (self.fragment)(uniforms, &varying, textures)
  }

  pub fn lerp_varyings(
    &self,
    bar: &Barycentric,
    varyings: &Varyings,
    rhws: [f32; 3],
    z: f32,
  ) -> Varying {
    let mut result = Varying::default();
    for key in varyings.data.keys() {
      let vec = varyings.data.get(key).unwrap();
      let length = vec.len();

      match length {
        1 => {
          let val = vec[0];
          result.set(key, val);
        }

        3 => {
          let arr = [vec[0] * rhws[0], vec[1] * rhws[1], vec[2] * rhws[2]];
          let lerped_val = bar.apply_weight(&arr) * z;
          result.set(key, lerped_val);
        }
        _ => continue,
      }
    }

    result
  }
}

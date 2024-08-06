use std::{
  collections::HashMap,
  fmt::Debug,
  ops::{Add, Mul},
};

use crate::{
  math::{Barycentric, Mat4, Vec2, Vec3, Vec4},
  model::Vertex,
};

use super::material::{GetTexture, Textures};

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
  };
}

define_union_type_enum!(
  GLTypes;
  Int@i32,
  Float@f32,
  Vec2@Vec2,
  Vec3@Vec3,
  Vec4@Vec4
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
    }
  }
}
pub struct GlMatrix<'a> {
  model_matrix: &'a Mat4,
  view_matrix: &'a Mat4,
  projection_matrix: &'a Mat4,
}

impl<'a> GlMatrix<'a> {
  pub fn new(model_matrix: &'a Mat4, view_matrix: &'a Mat4, projection_matrix: &'a Mat4) -> Self {
    Self {
      model_matrix,
      view_matrix,
      projection_matrix,
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
#[derive(Debug, Default)]
pub struct GlCollection {
  data: HashMap<String, GLTypes>,
}

impl GlCollection {
  pub fn set(&mut self, key: &str, gl_values: GLTypes) {
    self.data.insert(key.to_string(), gl_values);
  }

  pub fn get(&self, key: &str) -> Option<GLTypes> {
    self.data.get(key).map(|e| *e)
  }
}

type Uniform = GlCollection;
type Varying = GlCollection;

type VertexShader = Box<dyn Fn(&GlMatrix, &Vertex, &Uniform, &mut Varyings) -> Vertex>;
type FragmentShader = Box<dyn Fn(&Uniform, &Varying, &Textures) -> Vec4>;

pub struct Shader {
  uniforms: Uniform,
  pub vertex: VertexShader,
  pub fragment: FragmentShader,
}

impl Debug for Shader {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Shader")
      .field("uniforms", &self.uniforms)
      .field("vertex", &"/** vertex clousure */".to_string())
      .field("fragment", &"/** fragment clousure */".to_string())
      .finish()
  }
}
impl Default for Shader {
  fn default() -> Self {
    Self {
      uniforms: Default::default(),
      vertex: Self::default_vertex(),
      fragment: Self::default_fragment(),
    }
  }
}

impl Shader {
  pub fn default_vertex() -> VertexShader {
    let vertex: VertexShader = Box::new(|gl_matrix, v, u, vary| {
      let GlMatrix {
        model_matrix,
        view_matrix,
        projection_matrix,
      } = gl_matrix;
      let mut next_v = *v;
      next_v.position =
        (**projection_matrix) * (**view_matrix) * (**model_matrix) * next_v.position;

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
    gl_matrix: &GlMatrix,
    gl_vertex: &Vertex,
    varyings: &mut Varyings,
  ) -> Vertex {
    (self.vertex)(gl_matrix, gl_vertex, &self.uniforms, varyings)
  }

  pub fn run_fragment(
    &self,
    bar: &Barycentric,
    varyings: &Varyings,
    textures: &Textures,
    rhws: [f32; 3],
    z: f32,
  ) -> Vec4 {
    let varying = self.lerp_varyings(bar, varyings, rhws, z);

    (self.fragment)(&self.uniforms, &varying, textures)
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

      if vec.len() < 3 {
        println!("???");
        continue;
      }

      let arr = [vec[0] * rhws[0], vec[1] * rhws[1], vec[2] * rhws[2]];
      // let arr = [vec[0], vec[1], vec[2]];

      let lerped_val = bar.apply_weight(&arr) * z;

      result.set(key, lerped_val);
    }

    result
  }
}

use std::{collections::HashMap, fmt::Debug};

use crate::{
  math::{Barycentric, Mat4, Vec2, Vec3, Vec4},
  model::Vertex,
};
#[derive(Debug, Clone, Copy)]
pub enum GLTypes {
  Int(i32),
  Float(f32),
  Vec2(Vec2),
  Vec3(Vec3),
  Vec4(Vec4),
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
type FragmentShader = Box<dyn Fn(&Vertex, &Uniform, &Varying) -> Vec4>;

pub struct Shader {
  uniforms: Uniform,
  varyings: Varyings,
  pub vertex: VertexShader,
  pub fragment: FragmentShader,
}

impl Debug for Shader {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Shader")
      .field("uniforms", &self.uniforms)
      .field("varyings", &self.varyings)
      .field("vertex", &"/** vertex clousure */".to_string())
      .field("fragment", &"/** fragment clousure */".to_string())
      .finish()
  }
}
impl Default for Shader {
  fn default() -> Self {
    Self {
      uniforms: Default::default(),
      varyings: Default::default(),
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

  pub fn run_vertex(&mut self, gl_matrix: &GlMatrix, gl_vertex: &Vertex) -> Vertex {
    (self.vertex)(gl_matrix, gl_vertex, &self.uniforms, &mut self.varyings)
  }

  pub fn run_fragment(&self, gl_vertex: &Vertex, bar: &Barycentric) -> Vec4 {
    let varying = self.lerp_varyings();

    (self.fragment)(gl_vertex, &self.uniforms, &varying)
  }

  pub fn lerp_varyings(&self) -> Varying {
    let result = Varying::default();
    for key in self.varyings.data.keys() {}

    result
  }
}

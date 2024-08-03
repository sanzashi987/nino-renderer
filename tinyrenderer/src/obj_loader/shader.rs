use std::{collections::HashMap, fmt::Debug};

use crate::{
  math::{Mat4, Vec2, Vec3, Vec4},
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
pub struct GlCollection {
  data: HashMap<String, GLTypes>,
}

impl GlCollection {
  pub fn new() -> Self {
    Self {
      data: HashMap::new(),
    }
  }
  pub fn set(&mut self, key: &str, val: GLTypes) {
    self.data.insert(key.to_string(), val);
  }

  pub fn get(&self, key: &str) -> Option<GLTypes> {
    self.data.get(key).map(|e| *e)
  }
}

pub type Varying = GlCollection;
pub type Uniform = Varying;

type VertexShader = Box<dyn Fn(&GlMatrix, &Vertex, &Uniform, &mut Varying) -> Vertex>;
type FragmentShader = Box<dyn Fn(&Vertex, &Uniform, &Varying) -> Vec4>;

pub struct Shader {
  uniforms: Uniform,
  varyings: Varying,
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
}

use crate::core::uniform::u;
use crate::math::{Mat4, Vec3, Vec4};

use crate::core::buffer_attribute::a;
use crate::core::buffer_geometry::Attribute;

use crate::core::uniform::Uniform;
use crate::core::varying::Varying;
use std::fmt::Debug;

#[derive(Default)]
pub struct GlPerVertex {
  pub gl_position: Vec4,
  pub gl_point_size: f32,
  pub gl_clip_distance: Vec<f32>,
  pub rhw: f32,
}

#[derive(Default)]
pub struct GlPerFragment {
  pub gl_frag_color: Vec4,
}

pub type VertexShader = Box<dyn Fn(&Attribute, &Uniform, &mut Varying, &mut GlPerVertex)>;
pub type FragmentShader = Box<dyn Fn(&Uniform, &Varying, &mut GlPerFragment) -> bool>;

pub struct Shader {
  pub vertex: VertexShader,
  pub fragment: FragmentShader,
}

pub trait DefineShader {
  fn vertex() -> VertexShader;
  fn fragment() -> FragmentShader;
}

impl Debug for Shader {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Shader")
      .field("vertex", &"/** vertex clousure */")
      .field("fragment", &"/** fragment clousure */")
      .finish()
  }
}

impl Shader {
  pub fn default_vertex() -> VertexShader {
    let vertex: VertexShader = Box::new(|attribute, unifrom, _, gl| {
      let model_matrix = u!(unifrom, Mat4, "model_matrix", !);
      let view_matrix = u!(unifrom, Mat4, "view_matrix", !);
      let projection_matrix = u!(unifrom, Mat4, "projection_matrix", !);
      let position = Vec4::from_vec3(&a!(attribute, Vec3, "position", !), 1.0);
      gl.gl_position = projection_matrix * view_matrix * model_matrix * position;
    });
    vertex
  }
  pub fn default_fragment() -> FragmentShader {
    Box::new(|_, _, gl| {
      gl.gl_frag_color = Vec4::new(1.0, 1.0, 1.0, 1.0);
      true
    })
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

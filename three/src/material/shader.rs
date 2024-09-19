use crate::core::unifrom::u;
use crate::math::{Mat4, Vec3, Vec4};

use super::super::core::buffer_attribute::a;
use super::super::core::buffer_geometry::Attribute;

use super::super::core::unifrom::Uniform;
use super::super::core::varying::Varying;
use std::fmt::Debug;

macro_rules! define_gl_obj {
  ($name:tt, $($prop:tt:$ty:ty),+) => {
    #[derive(Debug, Default)]
    pub struct $name {
      $(
        $prop: $ty,
      )+
    }
    impl $name {

      $(
        pub fn $prop(&mut self, val:$ty){
          self.$prop = val
        }
      )+
      pub fn read(self) -> ($($ty),+){
        ($(self.$prop),+)
      }

    }
  };
}
define_gl_obj!(
  GlPerVertex,
  gl_position: Vec4,
  gl_point_size: f32,
  gl_clip_distance: Vec<f32>
);

define_gl_obj!(
  GlPerFragment,
  gl_frag_color: Vec4
);

type VertexShader = Box<dyn Fn(&Attribute, &Uniform, &mut Varying, &mut GlPerVertex)>;
type FragmentShader = Box<dyn Fn(&Uniform, &Varying, &mut GlPerFragment) -> bool>;

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

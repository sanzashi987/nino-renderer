use crate::math::{Vec3, Vec4};
use crate::obj_loader::shader::{GLTypes, Shader};

pub fn make_gouraud_shader(light_dir: Vec3) -> Shader {
  let mut shader = Shader::default();

  shader.vertex = Box::new(move |_, gl_vertex, _, varying| {
    if let Some(normal) = gl_vertex.normal {
      varying.set("light-intense", GLTypes::Float(normal.dot(&light_dir)))
    }

    *gl_vertex
  });


  shader.fragment = Box::new(|_, _, _| Vec4::zero());

  shader
}

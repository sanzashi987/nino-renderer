use crate::math::{Vec3, Vec4};
use crate::obj_loader::shader::{GLTypes, Shader};

pub fn make_gouraud_shader(light_dir: Vec3) -> Shader {
  let mut shader = Shader::default();
  let default_vertex = shader.vertex;
  shader.vertex = Box::new(move |gl_matrix, gl_vertex, u, varying| {
    if let Some(normal) = gl_vertex.normal {
      varying.set(
        "light-intense",
        GLTypes::Float(normal.dot(&light_dir.normalize()).max(0.0)),
      )
    }

    if let Some(uv) = gl_vertex.texture {
      varying.set("vUv", GLTypes::Vec2(uv));
    }

    default_vertex(gl_matrix, gl_vertex, u, varying)
  });

  shader.fragment = Box::new(|_, varying, textures| {
    let mut res = Vec4::zero();

    if let Some(val) = varying.get("light-intense") {
      if let GLTypes::Float(light_intense) = val {
        let s = (1.0 * light_intense).min(1.0);
        res = Vec4::new(s, s, s, 1.0);
      }
    };

    res
  });

  shader
}

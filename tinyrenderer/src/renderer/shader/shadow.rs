use crate::{
  math::{Vec3, Vec4},
  obj_loader::shader::{uniform, varying, Extract, GLTypes, Shader},
};

pub fn make_shadow_shader() -> Shader {
  let mut shader = Shader::default();
  let default_vertex = shader.vertex;

  shader.vertex = Box::new(move |vertex, uniforms, varyings| {
    if let Some(uv) = vertex.texture {
      varyings.set("vUv", GLTypes::Vec2(uv));
    }
    let vertex = default_vertex(vertex, uniforms, varyings);

    varyings.set("p", GLTypes::Vec4(vertex.position / vertex.position.w));

    vertex
  });

  shader.fragment = Box::new(|uniforms, varyings, textures| {
    let p = varying!(varyings, Vec4, "p", !);

    let color = Vec3::new(1.0, 1.0, 1.0) * p.z;

    Vec4::from_vec3(&color, 1.0)
  });

  shader
}

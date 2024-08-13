use crate::{
  math::{Vec3, Vec4},
  obj_loader::shader::{uniform, varying, Extract, GLTypes, Shader},
};

pub fn make_shadow_shader() -> Shader {
  let mut shader = Shader::default();

  shader.fragment = Box::new(|uniforms, varyings, textures| {
    let z = uniform!(uniforms, f32, "z", !);

    let color = Vec3::new(1.0, 1.0, 1.0) * z * (-1.0) / 40.0;

    Vec4::from_vec3(&color, 1.0)
  });

  shader
}

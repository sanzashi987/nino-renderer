use crate::math::{Vec2, Vec3, Vec4};
use crate::obj_loader::material::GetTexture;
use crate::obj_loader::shader::{Extract, GLTypes, Shader};

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
    let s = varying
      .get("light-intense")
      .map_or(None as Option<f32>, |v| v.extract())
      .map_or(1.0, |v| v.min(1.0));

    let vUv = varying
      .get("vUv")
      .map_or(None as Option<Vec2>, |v| v.extract());

    if let (Some(texture), Some(uv)) = (textures.get_texture_by_id(0), vUv) {
      let mut res = texture.get_pixel(uv) * s;
      res.w = 1.0;
      res
    } else {
      Vec4::new(s, s, s, 1.0)
    }
  });

  shader
}

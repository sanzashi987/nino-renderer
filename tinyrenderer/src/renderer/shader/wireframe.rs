use crate::obj_loader::shader::{uniform, varying, Extract, GLTypes, Shader};
use math::{Vec2, Vec3, Vec4};

/// Renders the mesh with flat material color and black wireframe edges.
/// Uses the barycentric coordinate technique: each vertex receives one component
/// of (1,0,0)/(0,1,0)/(0,0,1); fragments whose minimum component falls below
/// `edge_width` are drawn black.
pub fn make_wireframe_shader(edge_width: f32) -> Shader {
  let mut shader = Shader::default();
  let default_vertex = shader.vertex;

  shader.vertex = Box::new(move |gl_vertex, uniforms, varyings| {
    let i = uniform!(uniforms, f32, "vertex_index", !);
    let bary = match i as u32 {
      0 => Vec3::new(1.0, 0.0, 0.0),
      1 => Vec3::new(0.0, 1.0, 0.0),
      _ => Vec3::new(0.0, 0.0, 1.0),
    };
    varyings.set("bary", GLTypes::Vec3(bary));

    if let Some(uv) = gl_vertex.texture {
      varyings.set("vUv", GLTypes::Vec2(uv));
    }

    default_vertex(gl_vertex, uniforms, varyings)
  });

  shader.fragment = Box::new(move |_uniforms, varyings, textures| {
    let bary = varying!(varyings, Vec3, "bary", !);

    let on_edge = bary.x < edge_width || bary.y < edge_width || bary.z < edge_width;
    if on_edge {
      return Vec4::new(0.0, 0.0, 0.0, 1.0);
    }
    return Vec4::new(0.85, 0.65, 0.55, 1.0);

    // let vUv = varying!(varyings, Vec2, "vUv");
    // if let (Some(uv), Some(diffuse)) = (vUv, textures.get_texture_by_id(0)) {
    //   let mut c = diffuse.get_pixel(uv);
    //   c.w = 1.0;
    //   c
    // } else {
    //   Vec4::new(0.8, 0.8, 0.8, 1.0)
    // }
  });

  shader
}

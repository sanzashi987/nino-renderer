use crate::obj_loader::shader::{uniform, varying, Extract, GLTypes, Shader};
use math::{Mat4, Vec3, Vec4};

/// Renders the mesh with flat material color and black wireframe edges.
/// Uses the barycentric coordinate technique: each vertex receives one component
/// of (1,0,0)/(0,1,0)/(0,0,1); fragments whose minimum component falls below
/// `edge_width` are drawn black.
///
/// Since the OBJ may not have normal data, this shader computes face normals
/// from the three vertex positions and applies simple diffuse lighting.
pub fn make_wireframe_shader(edge_width: f32, light_dir: Vec3) -> Shader {
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

    // Store world position for normal calculation in fragment shader
    let model_matrix = uniform!(uniforms, Mat4, "model_matrix", !);
    let world_pos = model_matrix * gl_vertex.position;
    varyings.set(&format!("world_pos_{}", i as i32), GLTypes::Vec4(world_pos));

    default_vertex(gl_vertex, uniforms, varyings)
  });

  shader.fragment = Box::new(move |_uniforms, varyings, _textures| {
    let bary = varying!(varyings, Vec3, "bary", !);

    let on_edge = bary.x < edge_width || bary.y < edge_width || bary.z < edge_width;
    if on_edge {
      return Vec4::new(0.0, 0.0, 0.0, 1.0);
    }

    // Compute face normal from three world positions
    let p0 = varying!(varyings, Vec4, "world_pos_0", !);
    let p1 = varying!(varyings, Vec4, "world_pos_1", !);
    let p2 = varying!(varyings, Vec4, "world_pos_2", !);

    let v0 = p0.truncated_to_vec3();
    let v1 = p1.truncated_to_vec3();
    let v2 = p2.truncated_to_vec3();

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let normal = edge1.cross(&edge2).normalize();

    // Material properties from sample_0001.mtl
    let ka = Vec3::new(0.30, 0.22, 0.20); // ambient color
    let kd = Vec3::new(0.85, 0.65, 0.55); // diffuse color
    let ks = Vec3::new(0.15, 0.15, 0.15); // specular color
    let ns = 25.0; // specular exponent (shininess)

    // Light direction (normalized)
    let l = light_dir.normalize();

    // View direction - assume camera at origin looking at object
    // Use center of triangle as approximate surface point
    let center = (v0 + v1 + v2) / 3.0;
    let view_dir = (center * -1.0).normalize();

    // Ambient term
    let ambient = ka;

    // Diffuse term
    let diff_intensity = normal.dot(&l).max(0.0);
    let diffuse = kd * diff_intensity;

    // Specular term (Phong reflection) - only when light faces surface
    let specular = if diff_intensity > 0.0 {
      let r = (normal * (2.0 * normal.dot(&l)) - l).normalize();
      let spec_intensity = r.dot(&view_dir).max(0.0).powf(ns);
      ks * spec_intensity
    } else {
      Vec3::new(0.0, 0.0, 0.0)
    };

    // Combine lighting
    let color = ambient + diffuse + specular;

    Vec4::new(color.x, color.y, color.z, 1.0)
  });

  shader
}

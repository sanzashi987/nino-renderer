use crate::{
  math::{Mat3, Mat4, Vec2, Vec3, Vec4},
  obj_loader::shader::{uniform, varying, Extract, GLTypes, Shader},
};

pub fn make_phong_shader(light_dir: Vec3) -> Shader {
  let mut shader = Shader::default();
  let default_vertex = shader.vertex;

  shader.vertex = Box::new(move |gl_vertex, uniforms, varyings| {
    if let Some(uv) = gl_vertex.texture {
      varyings.set("vUv", GLTypes::Vec2(uv));
    }

    default_vertex(gl_vertex, uniforms, varyings)
  });

  shader.fragment = Box::new(move |uniforms, varyings, textures| {
    let vUv = varying!(varyings, Vec2, "vUv");
    let mv_it = uniform!(uniforms, Mat4, "mv_it", !);
    let mut color = Vec4::new(1.0, 1.0, 1.0, 1.0);

    if let Some(uv) = vUv {
      if let Some(diffuse) = textures.get_texture_by_id(0) {
        color = diffuse.get_pixel(uv);
      }

      let t = textures.get_texture_by_ids(vec![1, 3]);

      // let t = textures.get_texture_by_ids(vec![2, 3]);

      if let (Some(normal), Some(specular)) = (t[0], t[1]) {
        let mut nn = normal.get_pixel(uv);
        nn = nn * 2.0 - 1.0;
        // bgr ---> zyx ---> xyz
        std::mem::swap(&mut nn.x, &mut nn.z);

        // dbg!(*mit);
        let n = (*mv_it * nn).truncated_to_vec3().normalize();
        // let n = nn.truncated_to_vec3().normalize();
        // let l = (mvp * Vec4::from_vec3(&light_dir, 1.0))
        //   .truncated_to_vec3()
        //   .normalize();
        let l = light_dir.normalize();

        let r = (n * (n.dot(&l) * 2.0) - l).normalize();
        let spec_strength = r.z.max(0.0).powf(specular.get_pixel(uv).z * 255.0);

        let intense = n.dot(&l).max(0.0);

        // color = color * intense;
        // color.w = 1.0;
        color = Vec4::new(
          (5.0 + color.x * 255.0 * (intense + 1.6 * spec_strength)).min(255.0) / 255.0,
          (5.0 + color.y * 255.0 * (intense + 1.6 * spec_strength)).min(255.0) / 255.0,
          (5.0 + color.z * 255.0 * (intense + 1.6 * spec_strength)).min(255.0) / 255.0,
          color.w,
        );
      }

      // if let Some(normal) = textures.get_texture_by_id(1) {}
      // if let Some(normal_tan) = textures.get_texture_by_id(2) {}
      // if let Some(spec) = textures.get_texture_by_id(3) {}
    }

    color
  });

  shader
}

pub fn make_phong_shader_with_tangent_normal_map(light_dir: Vec3) -> Shader {
  let mut shader = Shader::default();
  let default_vertex = shader.vertex;
  shader.vertex = Box::new(move |gl_vertex, uniforms, varyings| {
    let i = uniform!(uniforms, f32, "vertex_index", !);

    if let Some(uv) = gl_vertex.texture {
      varyings.set("vUv", GLTypes::Vec2(uv));
      varyings.set(&format!("uv_{}", i), GLTypes::Vec2(uv));
    }

    if let Some(n) = gl_vertex.normal {
      varyings.set("normal", GLTypes::Vec3(n));
    }

    let v = default_vertex(gl_vertex, uniforms, varyings);
    let p = v.position;

    varyings.set(&format!("vertex_{}", i), GLTypes::Vec4(p / p.w));

    v
  });

  shader.fragment = Box::new(|uniforms, varyings, textures| {
    // let ma =

    let p0 = varying!(varyings, Vec4, "vertex_0", !);
    let p1 = varying!(varyings, Vec4, "vertex_1", !);
    let p2 = varying!(varyings, Vec4, "vertex_2", !);

    let uv0 = varying!(varyings, Vec2, "uv_0", !);
    let uv1 = varying!(varyings, Vec2, "uv_1", !);
    let uv2 = varying!(varyings, Vec2, "uv_2", !);

    let uv = varying!(varyings, Vec2, "vUv", !);
    let bn = varying!(varyings, Vec3, "normal", !);
    let mut A = Mat3::default();

    A.set_col(0, (*p1 - *p0).truncated_to_vec3());
    A.set_col(1, (*p2 - *p0).truncated_to_vec3());
    A.set_col(2, *bn);

    let AI = A.inverse().unwrap();

    let i: Vec3 = AI * Vec3::new(uv1.x - uv0.x, uv2.x - uv0.x, 0.0);
    let j: Vec3 = AI * Vec3::new(uv1.y - uv0.y, uv2.y - uv0.y, 0.0);

    let mut B = Mat3::default();
    B.set_col(0, i.normalize());
    B.set_col(1, j.normalize());
    B.set_col(2, *bn);

    Vec4::default()
  });

  shader
}

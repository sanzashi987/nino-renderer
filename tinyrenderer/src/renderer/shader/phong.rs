use std::iter::Inspect;

use crate::{
  math::{Mat4, Vec2, Vec3, Vec4},
  obj_loader::shader::{take_value, Extract, GLTypes, Shader},
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

  shader.fragment = Box::new(move |uniforms, varying, textures| {
    let vUv = varying
      .get("vUv")
      .map_or(None as Option<Vec2>, |v| v.extract().map(take_value));

    let mit: &Mat4 = uniforms.get("mvp_it").unwrap().extract().unwrap();
    let model_matrix: &Mat4 = uniforms.get("model_matrix").unwrap().extract().unwrap();
    let view_matrix: &Mat4 = uniforms.get("view_matrix").unwrap().extract().unwrap();
    let projection_matrix: &Mat4 = uniforms
      .get("projection_matrix")
      .unwrap()
      .extract()
      .unwrap();

    let mv = *projection_matrix * (*view_matrix) * (*model_matrix);

    let mut color = Vec4::new(1.0, 1.0, 1.0, 1.0);

    if let Some(uv) = vUv {
      if let Some(diffuse) = textures.get_texture_by_id(0) {
        color = diffuse.get_pixel(uv);
      }

      let t = textures.get_texture_by_ids(vec![1, 3]);

      // let t = textures.get_texture_by_ids(vec![2, 3]);

      if let (Some(normal), Some(spec)) = (t[0], t[1]) {
        let mut nn = normal.get_pixel(uv);
        nn = nn / 0.5 - 0.5;

        let n = (*mit * nn).truncated_to_vec3().normalize();

        let l = (mv * Vec4::from_vec3(&light_dir, 1.0))
          .truncated_to_vec3()
          .normalize();

        let r = (n * (n.dot(&l) * 2.0) - l).normalize();
        let spe = r.z.max(0.0).powf(spec.get_pixel(uv).x);
        let intense = n.dot(&l).max(0.0);

        color = color * intense;
        color.w = 1.0;
        // color = Vec4::new(
        //   (5.0 + color.x * 255.0 * (intense + 0.6 * spe)).min(255.0) / 255.0,
        //   (5.0 + color.y * 255.0 * (intense + 0.6 * spe)).min(255.0) / 255.0,
        //   (5.0 + color.z * 255.0 * (intense + 0.6 * spe)).min(255.0) / 255.0,
        //   color.w,
        // );
      }

      // if let Some(normal) = textures.get_texture_by_id(1) {}
      // if let Some(normal_tan) = textures.get_texture_by_id(2) {}
      // if let Some(spec) = textures.get_texture_by_id(3) {}
    }

    color
  });

  shader
}

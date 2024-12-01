use std::{default, rc::Rc};

use crate::{
  cameras::camera::ICamera,
  core::{
    buffer_attribute::{ToF32, TypeBufferAttribute, TypeBufferEnum},
    buffer_geometry::{pick_attribute_per_vertex, Attribute, IGeometry},
    object_3d::{IObject3D, ObjectType},
    render_target::RenderTarget,
    uniform::Uniform,
    varying::Varying,
  },
  material::{material::IMaterial, shader::GlPerVertex},
  math::Vec2,
};

enum RenderMode {
  Triangle,
  Point,
  Line,
}

fn iter_triangle_verterx(attr: &Attribute, group: usize) -> [Attribute; 3] {
  let first = pick_attribute_per_vertex(attr, group);
  let second = pick_attribute_per_vertex(attr, group + 1);
  let third = pick_attribute_per_vertex(attr, group + 2);
  return [first, second, third];
}

fn render_triangle<T: Sized + Copy + ToF32>(
  target: &RenderTarget,
  position: &Box<TypeBufferAttribute<T>>,
  attribute: &Attribute,
  material: Rc<dyn IMaterial>,
  uniform: &Uniform,
) {
  let data = &position.data;
  let size = &position.size;
  let num_of_vertex = data.len() / size;
  for i in 0..num_of_vertex / 3_usize {
    let index = i * 3_usize;
    let vertex_attribute = iter_triangle_verterx(attribute, index);
    let mut varyings = Varying::default();

    let mut vs_results: [GlPerVertex; 3] = Default::default();
    for j in 0..3 {
      material.vertex(
        &vertex_attribute[j],
        uniform,
        &mut varyings,
        &mut vs_results[j],
      );
    }

    let mut vertices_2d: [Vec2; 3] = Default::default();
    for j in 0..3 {
      vs_results[j].rhw = 1.0 / vs_results[j].gl_position.w;
      vs_results[j].gl_position /= vs_results[j].gl_position.w;

      vs_results[j].gl_position = *viewport_matrix * vs_results[j].gl_position;

      vertices_2d[j] = vs_results[j].gl_position.truncate_to_vec2();
    }
  }
}

pub fn render_pipeline(
  target: &RenderTarget,
  camera: Rc<dyn ICamera>,
  object: Rc<dyn IObject3D>,
  geometry: Rc<dyn IGeometry>,
  material: Rc<dyn IMaterial>,
  vertex_pointer: Option<String>,
) {
  let mut mode = RenderMode::Triangle;

  match object.get_type() {
    ObjectType::Line => mode = RenderMode::Line,
    ObjectType::Point => mode = RenderMode::Point,
    _ => {}
  }

  if material.wireframe() {
    mode = RenderMode::Line;
  }

  let pointer = &vertex_pointer.unwrap_or("position".to_string());

  match mode {
    RenderMode::Triangle => {
      let attribute = geometry.get_attribute();
      let position = attribute.get(pointer);
      let uniform = material.to_uniform();
      if let Some(p) = position {
        match p {
          TypeBufferEnum::F64(buffer) => {
            render_triangle(target, buffer, attribute, material.clone(), &uniform)
          }
          TypeBufferEnum::F32(buffer) => {
            render_triangle(target, buffer, attribute, material.clone(), &uniform)
          }
          _ => {}
        }
      }
    }
    RenderMode::Point => todo!(),
    RenderMode::Line => todo!(),
  }
}

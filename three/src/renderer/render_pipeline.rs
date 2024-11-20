use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  core::{
    buffer_attribute::TypeBufferEnum, buffer_geometry::IGeometry, object_3d::IObject3D,
    render_target::RenderTarget,
  },
  material::material::IMaterial,
  objects::base::Renderable,
};

enum RenderMode {
  Triangle,
  Point,
  Line,
}

pub fn render_pipeline(
  target: &RenderTarget,
  camera: Rc<dyn ICamera>,
  object: Rc<dyn IObject3D>,
  geometry: Rc<dyn IGeometry>,
  material: Rc<dyn IMaterial>,
) {
  let mut mode = RenderMode::Triangle;

  match object.get_type() {
    crate::core::object_3d::ObjectType::Line => mode = RenderMode::Line,
    crate::core::object_3d::ObjectType::Point => mode = RenderMode::Point,
    _ => {}
  }

  if material.wireframe() {
    mode = RenderMode::Line;
  }

  match mode {
    RenderMode::Triangle => {
      let position = geometry.get_attribute().get("position");
      if let Some(p) = position {
        match p {
          TypeBufferEnum::F64(type_buffer_attribute) => {
            let data = &type_buffer_attribute.data;
            for i in 0..data.len() {
              let index = (i * 3) as usize;
              let mut vertices = [data[index], data[index + 1], data[index + 2]];
              for v in &mut vertices {
                // uniforms.set("vertex_index", GLTypes::Float(index as f32));
                *v = shader.run_vertex(v, &uniforms, &mut varyings);
                index += 1.0;
              }
            }
          }
          TypeBufferEnum::F32(type_buffer_attribute) => {}
          _ => {}
        }
      }
    }
    RenderMode::Point => todo!(),
    RenderMode::Line => todo!(),
  }
}

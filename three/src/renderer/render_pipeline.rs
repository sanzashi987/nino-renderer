use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  core::{
    buffer_attribute::TypeBufferEnum,
    buffer_geometry::{Attribute, IGeometry},
    object_3d::IObject3D,
    render_target::RenderTarget,
    varying::Varying,
  },
  material::material::IMaterial,
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
      let attribute = geometry.get_attribute();
      let position = attribute.get("position");
      let uniform = material.to_uniform();
      if let Some(p) = position {
        match p {
          TypeBufferEnum::F64(type_buffer_attribute) => {
            let data = &type_buffer_attribute.data;
            let size = &type_buffer_attribute.size;
            let mut index = 0;
            loop {
              let mut attribute_per_vertex = Attribute::default();

              attribute.iter().for_each(|(k, v)| {
                // v.
                attribute_per_vertex.insert(k.to_string(), v);
              });
              index += 1;
            }

            for i in 0..data.len() / 3 {
              let index = (i * 3) as usize;
              let mut vertices = [data[index], data[index + 1], data[index + 2]];
              let mut varyings = Varying::default();

              for v in &mut vertices {
                let mut gl_vertex = Default::default();
                material.vertex(attribute, &uniform, &mut varyings, &mut gl_vertex);
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

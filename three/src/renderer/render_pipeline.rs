use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  core::{buffer_geometry::IGeometry, object_3d::IObject3D, render_target::RenderTarget},
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
}

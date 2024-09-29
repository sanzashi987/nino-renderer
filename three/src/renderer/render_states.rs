use std::rc::Rc;

use crate::{
  core::{buffer_geometry::GeometryActions, object_3d::ObjectActions},
  material::material::ConvertUniform,
};
#[derive(Default)]
pub struct RenderStates {
  lights: Vec<Rc<dyn ObjectActions>>,
  shadows: Vec<Rc<dyn ObjectActions>>,
}

impl RenderStates {
  pub fn push_light(&mut self, light: Rc<dyn ObjectActions>) {
    self.lights.push(light)
  }

  pub fn push_shadow(&mut self, light: Rc<dyn ObjectActions>) {
    self.shadows.push(light)
  }
}

trait EntityList: ObjectActions + GeometryActions + ConvertUniform {}

pub type RenderList = Vec<Rc<dyn EntityList>>;

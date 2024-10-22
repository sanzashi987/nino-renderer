use std::{collections::HashMap, rc::Rc};

use crate::{
  cameras::camera::{self, Camera},
  core::{buffer_geometry::IGeometry, object_3d::ObjectActions},
  material::material::MaterialActions,
};
// #[derive(Default)]
pub struct RenderState {
  lights: Vec<Rc<dyn ObjectActions>>,
  shadows: Vec<Rc<dyn ObjectActions>>,
  camera: Rc<dyn Camera>,
}

impl RenderState {
  pub fn push_light(&mut self, light: Rc<dyn ObjectActions>) {
    self.lights.push(light)
  }

  pub fn push_shadow(&mut self, light: Rc<dyn ObjectActions>) {
    self.shadows.push(light)
  }

  pub fn init(&mut self, camera: Rc<dyn Camera>) {
    self.camera = camera.clone();
  }
}

trait EntityList: ObjectActions + IGeometry + MaterialActions {}

pub type RenderList = Vec<Rc<dyn EntityList>>;

pub struct RenderStates {
  map: HashMap<String, RenderState>,
}

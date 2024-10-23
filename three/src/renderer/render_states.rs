use std::{collections::HashMap, rc::Rc};

use crate::{
  cameras::camera::{self, Camera},
  core::{buffer_geometry::IGeometry, object_3d::ObjectActions},
  material::material::IMaterial,
};
#[derive(Default)]
pub struct RenderState {
  lights: Vec<Rc<dyn ObjectActions>>,
  shadows: Vec<Rc<dyn ObjectActions>>,
  camera: Option<Rc<dyn Camera>>,
}

impl RenderState {
  pub fn push_light(&mut self, light: Rc<dyn ObjectActions>) {
    self.lights.push(light)
  }

  pub fn push_shadow(&mut self, light: Rc<dyn ObjectActions>) {
    self.shadows.push(light)
  }

  pub fn init(&mut self, camera: Rc<dyn Camera>) {
    self.camera = Some(camera.clone());
    self.lights = vec![];
    self.shadows = vec![];
  }
}

#[derive(Default)]
pub struct RenderStates {
  map: HashMap<String, RenderState>,
}

struct RenderItem {
  id: String,
  object: Option<Rc<dyn ObjectActions>>,
  geometry: Option<Rc<dyn IGeometry>>,
  material: Option<Rc<dyn IMaterial>>,
  group: Option<Rc<dyn ObjectActions>>,
}

pub struct RenderList {
  index: u32,
  opaque: Vec<RenderItem>,
  transparent: Vec<RenderItem>,
  transmissive: Vec<RenderItem>,
}

impl RenderList {
  pub fn init(&mut self) {}
}

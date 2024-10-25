use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
  cameras::camera::{self, ICamera},
  core::{buffer_geometry::IGeometry, object_3d::ObjectActions},
  material::material::IMaterial,
};
#[derive(Default)]
pub struct RenderState {
  lights: RefCell<Vec<Rc<dyn ObjectActions>>>,
  shadows: RefCell<Vec<Rc<dyn ObjectActions>>>,
  camera: RefCell<Option<Rc<dyn ICamera>>>,
}

impl RenderState {
  pub fn push_light(&self, light: Rc<dyn ObjectActions>) {
    self.lights.borrow_mut().push(light);
  }

  pub fn push_shadow(&self, light: Rc<dyn ObjectActions>) {
    self.shadows.borrow_mut().push(light);
  }

  pub fn init(&self, camera: Rc<dyn ICamera>) {
    let mut c = self.camera.borrow_mut();

    *c = Some(camera.clone());
    let mut lights = self.lights.borrow_mut();
    let mut shadows = self.shadows.borrow_mut();

    *lights = vec![];
    *shadows = vec![];
  }
}

#[derive(Default)]
struct RenderItem {
  id: String,
  object: Option<Rc<dyn ObjectActions>>,
  geometry: Option<Rc<dyn IGeometry>>,
  material: Option<Rc<dyn IMaterial>>,
  group: Option<Rc<dyn ObjectActions>>,
}

#[derive(Default)]
pub struct RenderList {
  index: u32,
  opaque: Vec<RenderItem>,
  transparent: Vec<RenderItem>,
  transmissive: Vec<RenderItem>,
}

impl RenderList {
  pub fn init(&mut self) {}
}

pub struct RenderTypes<T: Default> {
  map: HashMap<String, T>,
}

impl<T: Default> RenderTypes<T> {
  pub fn get(&mut self, uuid: &str) -> &T {
    if !self.map.contains_key(uuid) {
      self.map.insert(uuid.to_string(), Default::default());
    }

    self.map.get(uuid).unwrap()
  }
}

impl<T: Default> Default for RenderTypes<T> {
  fn default() -> Self {
    Self {
      map: Default::default(),
    }
  }
}

pub type RenderStates = RenderTypes<RenderState>;
pub type RenderLists = RenderTypes<RenderList>;

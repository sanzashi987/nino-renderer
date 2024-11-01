use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

use crate::{
  cameras::camera::{self, ICamera},
  core::{buffer_geometry::IGeometry, object_3d::IObject3D},
  lights::light::ILight,
  material::material::IMaterial,
};
#[derive(Default)]
pub struct RenderState {
  lights: RefCell<Vec<Rc<dyn ILight>>>,
  shadows: RefCell<Vec<Rc<dyn ILight>>>,
  camera: RefCell<Option<Rc<dyn ICamera>>>,
}

impl RenderState {
  pub fn push_light(&self, light: Rc<dyn ILight>) {
    self.lights.borrow_mut().push(light);
  }

  pub fn push_shadow(&self, light: Rc<dyn ILight>) {
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

struct RenderItem {
  id: String,
  object: Rc<dyn IObject3D>,
  geometry: Rc<dyn IGeometry>,
  material: Rc<dyn IMaterial>,
  group_order: i32,
  z: f32,
  parent: Option<Rc<dyn IObject3D>>,
}

#[derive(Default)]
pub struct RenderList {
  index: u32,
  opaque: RefCell<Vec<Rc<RenderItem>>>,
  transparent: RefCell<Vec<Rc<RenderItem>>>,
  transmissive: RefCell<Vec<Rc<RenderItem>>>,
  render_items: RefCell<Vec<Rc<RenderItem>>>,
}

impl RenderList {
  pub fn push(
    &self,
    object: Rc<dyn IObject3D>,
    geometry: Rc<dyn IGeometry>,
    material: Rc<dyn IMaterial>,
    group_order: i32,
    z: f32,
    parent: Option<Rc<dyn IObject3D>>,
  ) {
    let render_item = RenderItem {
      id: object.uuid().to_string(),
      object,
      geometry,
      material,
      group_order,
      z,
      parent,
    };

    let render_item = Rc::new(render_item);

    if let Some(_) = render_item.material.transmission() {
      let mut transmissive = self.transmissive.borrow_mut();
      transmissive.push(render_item.clone());
    } else if render_item.material.transparent() {
      let mut transparent = self.transparent.borrow_mut();
      transparent.push(render_item.clone());
    } else {
      let mut opaque = self.opaque.borrow_mut();
      opaque.push(render_item.clone());
    }

    let mut render_items = self.render_items.borrow_mut();
    render_items.push(render_item.clone());
  }

  pub fn finish(&self) {
    let mut render_items = self.render_items.borrow_mut();
    *render_items = vec![];
  }
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

use std::rc::Rc;

use crate::{cameras::camera::ICamera, core::object_3d::ObjectActions, objects::scene::Scene};

#[derive(Debug, Default)]
pub struct ShadowMap {
  enable: bool,
}

impl ShadowMap {
  pub fn render(
    &self,
    lights: &Vec<Rc<dyn ObjectActions>>,
    scene: Rc<Scene>,
    camera: Rc<dyn ICamera>,
  ) {
  }
}

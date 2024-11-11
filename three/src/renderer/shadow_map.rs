use std::rc::Rc;

use crate::{cameras::camera::ICamera, lights::light::ILight, math::Vec2, objects::scene::Scene};

#[derive(Debug, Default)]
pub struct ShadowMap {
  enable: bool,
}

impl ShadowMap {
  pub fn render(&self, lights: &Vec<Rc<dyn ILight>>, scene: Rc<Scene>, camera: Rc<dyn ICamera>) {
    if !self.enable {
      return;
    }

    for light in lights {
      if let Some(shadow) = light.shadow() {
        let Vec2 {
          x: map_width,
          y: height,
        } = shadow.map_size();

        let vps = shadow.viewports();
        for vp in vps {}
      }
    }
  }
}

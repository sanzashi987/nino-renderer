use super::super::cameras;
use super::super::objects::scene::Scene;
use super::viewport::Viewport;
use crate::{
  data_array::{ColorBuffer, DepthBuffer},
  next::cameras::Camera,
};
pub struct GlRenderer {
  viewport: Viewport,
  color: ColorBuffer,
  depth: DepthBuffer,
}

impl GlRenderer {
  pub fn new() -> Self {
    Self {
      viewport: Default::default(),
      color: Default::default(),
      depth: Default::default(),
    }
  }

  pub fn set_size(&mut self, w: f32, h: f32) {
    self.viewport.set_size(w, h);
    self.color = ColorBuffer::new(w as u32, h as u32);
    self.depth = DepthBuffer::new(w as u32, h as u32);
  }

  // pub fn set_pixel_ratio(&mut self, r: f32) {}

  fn take_color(&mut self) -> ColorBuffer {
    let w = self.color.width();
    let h = self.color.height();
    self.depth.clear(std::f32::MAX);
    std::mem::replace(&mut self.color, ColorBuffer::new(w, h))
  }

  pub fn render(&mut self, scene: Scene, camera: impl Camera) -> ColorBuffer {
    self.take_color()
  }
}

fn recursive_render(color: &mut ColorBuffer) {



  
}

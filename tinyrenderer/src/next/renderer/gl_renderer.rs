use std::cell::RefCell;

pub use super::viewport::Viewport;
use crate::data_array::{ColorBuffer, DepthBuffer};
pub struct GlRenderer {
  viewport: Viewport,
  color: RefCell<ColorBuffer>,
  depth: RefCell<DepthBuffer>,
}

impl GlRenderer {
  pub fn new() -> Self {
    Self {
      viewport: Default::default(),
      // color:
    }
  }

  pub fn set_size(&mut self, w: f32, h: f32) {}

  pub fn set_pixel_ratio(&mut self, r: f32) {}

  pub fn take_color(&self) -> ColorBuffer {}
}

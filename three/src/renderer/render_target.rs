use std::cell::RefCell;

use crate::math::data_array::ColorBuffer;

use super::viewport::Viewport;
#[derive(Debug, Default)]
pub struct RenderTarget {
  viewport: RefCell<Viewport>,
  color: RefCell<ColorBuffer>,
}

impl RenderTarget {
  pub fn set_size(&self, w: f32, h: f32) {
    let mut viewport = self.viewport.borrow_mut();
    let mut color = self.color.borrow_mut();
    viewport.set_size(w, h);
    *color = ColorBuffer::new(w as u32, h as u32);
  }

  pub fn take_color(&self) -> ColorBuffer {
    let w = { self.color.borrow().width() };
    let h = { self.color.borrow().height() };
    let mut color = self.color.borrow_mut();

    std::mem::replace(&mut color, ColorBuffer::new(w, h))
  }
}

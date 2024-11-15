use std::cell::RefCell;

use crate::{
  math::data_array::ColorBuffer,
  textures::texture::{self, Texture},
};

use super::viewport::Viewport;
#[derive(Debug, Default)]
pub struct RenderTarget {
  viewport: RefCell<Viewport>,
  color: RefCell<ColorBuffer>,
  pub texture: RefCell<Texture>,
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

  pub fn update_texture(&self, texture: Texture) {
    let mut t = self.texture.borrow_mut();
    *t = texture;
  }

  pub fn update_texture_name(&self, name: String) {
    let mut t = self.texture.borrow_mut();
    t.name = name;
  }
}

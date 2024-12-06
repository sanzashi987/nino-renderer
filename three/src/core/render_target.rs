use std::cell::{Ref, RefCell};

use crate::{
  math::{data_array::ColorBuffer, Mat4, Vec4},
  textures::texture::Texture,
};

use super::viewport::Viewport;
#[derive(Debug, Default)]
pub struct RenderTarget {
  viewport: RefCell<Viewport>,
  texture: RefCell<Texture>,
}

impl RenderTarget {
  pub fn set_size(&self, w: f32, h: f32) {
    let mut viewport = self.viewport.borrow_mut();
    let mut texture = self.texture.borrow_mut();
    viewport.set_size(w, h);
    texture.set_size(w as u32, h as u32)

    // texture.
  }

  pub fn take_color(&self) -> Vec<u8> {
    let mut texture = self.texture.borrow_mut();
    texture.take_color().0
  }

  pub fn write(&self, x: u32, y: u32, color: Vec4) {
    let mut color_buffer = self.texture.borrow_mut();
    color_buffer.write(x, y, color);
  }

  pub fn update_texture(&self, texture: Texture) {
    let mut t = self.texture.borrow_mut();
    *t = texture;
  }

  pub fn update_texture_name(&self, name: String) {
    let mut t = self.texture.borrow_mut();
    t.name = name;
  }

  pub fn texture(&self) -> Ref<'_, Texture> {
    self.texture.borrow()
  }

  pub fn viewport(&self) -> Ref<'_, Viewport> {
    self.viewport.borrow()
  }

  pub fn update_and_get_viewport(&self) -> Mat4 {
    let mut v = self.viewport.borrow_mut();
    v.recompute_matrix();
    *v.get_viewport_matrix()
  }
}

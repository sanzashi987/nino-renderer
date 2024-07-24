use crate::data_array::{ColorBuffer, DepthBuffer};

pub struct Renderer {
  color: ColorBuffer,
  depth: DepthBuffer,
}

impl Renderer {
  pub fn new(w: u32, h: u32) -> Self {
    Self {
      // camera:
      color: ColorBuffer::new(w, h),
      depth: DepthBuffer::new(w, h),
    }
  }

  pub fn render(&mut self) {}
}

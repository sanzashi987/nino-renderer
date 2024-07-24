use super::camera::Camera;
use crate::{
  data_array::{ColorBuffer, DepthBuffer},
  math::Mat4,
};

pub struct Viewport {
  x: f32,
  y: f32,
  w: f32,
  h: f32,
  viewport_matrix: Mat4,
}

impl Viewport {
  pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
    let mut viewport = Self {
      x,
      y,
      w,
      h,
      viewport_matrix: Mat4::identity(),
    };

    viewport.recompute_matrix();

    viewport
  }

  pub fn recompute_matrix(&mut self) {}
}

pub struct Renderer {
  viewport: Viewport,
  camera: Camera,
  color: ColorBuffer,
  depth: DepthBuffer,
}

impl Renderer {
  pub fn new(w: u32, h: u32) -> Self {
    Self {
      viewport: Viewport::new(0.0, 0.0, w as f32, h as f32),
      camera: Camera::new(w as f32, h as f32),
      color: ColorBuffer::new(w, h),
      depth: DepthBuffer::new(w, h),
    }
  }

  pub fn render(&mut self) {}
}

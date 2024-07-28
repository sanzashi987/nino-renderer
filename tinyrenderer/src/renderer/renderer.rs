use super::camera::Camera;
use crate::{
  data_array::{ColorBuffer, DepthBuffer},
  math::Mat4,
  model::Scene,
};
/// It means that the bi-unit cube [-1,1]*[-1,1]*[-1,1]
/// is mapped onto the screen cube [x,x+w]*[y,y+h]*[0,d].
/// Right, cube, and not a rectangle,
/// this is because of the depth computations with the z-buffer.
///  Here d is the resolution of the z-buffer.
/// I like to have it equal to 255 because of simplicity of
/// dumping black-and-white images of the z-buffer for debugging.
pub struct Viewport {
  x: f32,
  y: f32,
  w: f32,
  h: f32,
  d: f32,
  viewport_matrix: Mat4,
}

impl Viewport {
  pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
    let mut viewport = Self {
      x,
      y,
      w,
      h,
      d: 1.0, //255.0,
      viewport_matrix: Mat4::identity(),
    };

    viewport.recompute_matrix();

    viewport
  }

  #[rustfmt::skip]
  pub fn recompute_matrix(&mut self) {
    let half_w = self.w/2.0;
    let half_h = self.h/2.0;
    let half_d = self.d/2.0;

    self.viewport_matrix = Mat4::from_row(&[
     half_w , 0.0    , 0.0   , self.x + half_w,
     0.0    , half_h , 0.0   , self.y + half_h,
     0.0    , 0.0    , half_d, half_d,
     0.0    , 0.0    , 0.0   , 1.0
    ]);
  }

  pub fn get_viewport_matrix(&self) -> &Mat4 {
    &self.viewport_matrix
  }
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

  pub fn render(&mut self, scene: &Scene, model_matrix: &Mat4) {
    let frustum = self.camera.get_frustum();
    let view_matrix = self.camera.get_view_matarix();
    let projection_matrix = frustum.get_projection_matrix();
    let viewport_matrix = self.viewport.get_viewport_matrix();

    for model in &scene.models {
      let vertices = &model.vertices;
      for i in 0..vertices.len() / 3_usize {
        let index = (i * 3) as usize;
        let mut vertices = [vertices[index], vertices[index + 1], vertices[index + 2]];

        for v in &mut vertices {
          v.position = *model_matrix * v.position;
        }
        for v in &mut vertices {
          v.position = *view_matrix * v.position;
        }
        for v in &mut vertices {
          v.position = *projection_matrix * v.position;
        }
        // restore the x,y,z  with 1/w, as the computation times `w` before

        for v in &mut vertices {
          v.position.z = -v.position.w;
          v.position.x /= v.position.w;
          v.position.y /= v.position.w;
        }

        for v in &mut vertices {
          v.position = *viewport_matrix * v.position;
        }
      }
    }
  }
}

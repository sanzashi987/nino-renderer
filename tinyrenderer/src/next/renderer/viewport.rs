use crate::math::Mat4;

/// It means that the bi-unit cube [-1,1]*[-1,1]*[-1,1]
/// is mapped onto the screen cube [x,x+w]*[y,y+h]*[0,d].
/// Right, cube, and not a rectangle,
/// this is because of the depth computations with the z-buffer.
///  Here d is the resolution of the z-buffer.
/// I like to have it equal to 255 because of simplicity of
/// dumping black-and-white images of the z-buffer for debugging.
#[derive(Debug, Default)]
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
      d: 1.0,
      viewport_matrix: Mat4::identity(),
    };

    viewport.recompute_matrix();
    // println!("{:?}", viewport.viewport_matrix);
    viewport
  }

  pub fn set_size(&mut self, w: f32, h: f32) {
    self.w = w;
    self.h = h;
    self.recompute_matrix();
  }


  #[rustfmt::skip]
  pub fn recompute_matrix(&mut self) {
    let half_w = self.w/2.0;
    let half_h = self.h/2.0;
    let half_d = self.d/2.0;

    self.viewport_matrix = Mat4::from_row(&[
     half_w , 0.0     , 0.0   , self.x + half_w,
     0.0    , -half_h , 0.0   , self.y + half_h,
     0.0    , 0.0     , half_d, half_d,
     0.0    , 0.0     , 0.0   , 1.0
    ]);
  }

  pub fn get_viewport_matrix(&self) -> &Mat4 {
    &self.viewport_matrix
  }
}

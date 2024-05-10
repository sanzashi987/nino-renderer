use crate::math::Mat4;
pub struct Frustum {
  near: f32,
  aspect: f32,
  fov: f32,

  mat: Mat4,
}

impl Frustum {
  #[rustfmt::skip]
  pub fn new(near: f32, aspect: f32, fov: f32) -> Frustum {
    let a = 1.0 / (near * fov.tan());
    Self{
      near,
      aspect,
      fov,
      mat: Mat4::from_row(&[
        a,     0.0,     0.0,     0.0,
        0.0, aspect*a,  0.0,     0.0,
        0.0,   0.0,     1.0,     0.0,
        0.0,   0.0,  -1.0/near,  0.0,
      ])
    }
  }

  pub fn get_mat(&self) -> &Mat4 {
    &self.mat
  }
}

pub struct Camera {
  frustum: Frustum,
}

impl Camera {
  pub fn new(near: f32, aspect: f32, fov: f32) -> Self {
    Self {
      frustum: Frustum::new(near, aspect, fov),
    }
  }

  pub fn get_frustum(&self) -> &Frustum {
    &self.frustum
  }
}

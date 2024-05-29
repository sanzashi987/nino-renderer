use crate::math::Mat4;
pub struct Frustum {
  near: f32,
  aspect: f32,
  fov: f32,

  mat: Mat4,
}

impl Frustum {
  #[rustfmt::skip]
  pub fn new(near: f32,far: f32, aspect: f32, fov: f32) -> Frustum {
    Self{
      near,
      aspect,
      fov,
      mat:if cfg!(feature="cpu"){
          let a = 1.0 / (near * fov.tan());
          // without far plane, clamp x,y in [-1, 1]^2, z= near
          Mat4::from_row(&[
            a,     0.0,     0.0,     0.0,
            0.0, aspect*a,  0.0,     0.0,
            0.0,   0.0,     1.0,     0.0,
            0.0,   0.0,  -1.0/near,  0.0,
          ])
        } else {
          let half_w = near * fov.tan();
          let half_h = half_w / aspect;
          // with far plane, clamp x,y,z in [-1, 1]^3
          Mat4::from_row(&[
            near / half_w,           0.0,                       0.0,                             0.0,
                      0.0, near / half_h,                       0.0,                             0.0,
                      0.0,           0.0, far + near / (far - near), 2.0 * far * near / (far - near),
                      0.0,           0.0,                      -1.0,                             0.0,
          ])
        }
    }
  }

  pub fn get_mat(&self) -> &Mat4 {
    &self.mat
  }

  pub fn near(&self) -> f32 {
    self.near
  }
}

pub struct Camera {
  frustum: Frustum,
}

impl Camera {
  pub fn new(near: f32, far: f32, aspect: f32, fov: f32) -> Self {
    Self {
      frustum: Frustum::new(near, far, aspect, fov),
    }
  }

  pub fn get_frustum(&self) -> &Frustum {
    &self.frustum
  }
}

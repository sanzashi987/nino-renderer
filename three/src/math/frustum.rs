use super::{Mat4, Vec3};

pub struct Frustum {
  near: f32,
  far: f32,
  aspect: f32,
  fov: f32,
  mat: Mat4,
}

impl Frustum {
  #[rustfmt::skip]
  pub fn new(near: f32, far: f32, aspect: f32, fov: f32) -> Self {
    let half_w = near * fov.tan();
    let half_h = half_w / aspect;
    let near = near.abs();
    let far = far.abs();
    Self {
      near,
      far,
      aspect,
      fov,
      mat: if cfg!(feature="cpu") {
        let a = 1.0 / (near * fov.tan());
        // without far plane, clamp x,y in [-1, 1], z = near
        Mat4::from_row(&[
          a,          0.0,         0.0, 0.0,
          0.0, aspect * a,         0.0, 0.0,
          0.0,        0.0,         1.0, 0.0,
          0.0,        0.0, -1.0 / near, 0.0,
        ])
      } else {
        Mat4::from_row(&[
          near / half_w,           0.0,                       0.0,                               0.0,
          0.0, near / half_h,                       0.0,                               0.0,
          0.0,           0.0, (far + near) / (near - far), 2.0 * far * near / (near - far),
          0.0,           0.0,                      -1.0,                               0.0,
        ])
      }
    }
  }

  pub fn get_projection_matrix(&self) -> &Mat4 {
    &self.mat
  }
  pub fn contains(&self, pt: &Vec3) -> bool {
    let half_width = self.near * self.fov.tan();
    let half_height = half_width / self.aspect;
    // let h_fovy_cos = self.fov.cos();
    // let h_fovy_sin = self.fov.sin();

    // right  plane normal (half_width,0,-near) x (0, 1, 0) = (near , 0 , half_width)
    // left   plane normal (-half_width, 0, -near) x ((0, -1, 0)  = (-near, 0 , half_width)
    // top    plane normal (0, half_height, -near) x (-1, 0, 0) = (0 , near, half_heigth)
    // bottom plane normal (0, -half_height, -near) x (1, 0, 0) = (0 , -near, half_heigth)
    !(Vec3::new(self.near, 0.0, half_width).dot(pt) >= 0.0
      || Vec3::new(-self.near, 0.0, half_width).dot(pt) >= 0.0
      || Vec3::new(0.0, self.near, half_height).dot(pt) >= 0.0
      || Vec3::new(0.0, -self.near, half_height).dot(pt) >= 0.0
      || pt.z >= -self.near
      || pt.z <= -self.far)
  }
}

// impl From<Mat4> for Frustum {
//   fn from(value: Mat4) -> Self {
//     todo!()
//   }
// }

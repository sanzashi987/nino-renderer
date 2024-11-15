use std::rc::Rc;

use crate::core::object_3d::IObject3D;

use super::{Mat4, Vec3, Vec4};

pub struct PerspectiveFrustum {
  near: f32,
  far: f32,
  aspect: f32,
  fov: f32,
  mat: Mat4,
}

impl PerspectiveFrustum {
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
#[derive(Debug, Default)]
pub struct Plane {
  constant: f32,
  normal: Vec3,
}

impl Plane {
  fn set_components(&mut self, x: f32, y: f32, z: f32, constant: f32) {
    self.normal.x = x;
    self.normal.y = y;
    self.normal.z = z;
    self.constant = constant;

    self.normalize();
  }

  fn normalize(&mut self) {
    let length = self.normal.length();
    self.normal.normalize();
    self.constant /= length;
  }
}
#[derive(Debug, Default)]
pub struct Frustum {
  planes: [Plane; 6],
}

impl Frustum {
  pub fn new(planes: [Plane; 6]) -> Self {
    Self { planes }
  }

  pub fn from_projection_matrix(projection_matrix: Mat4) -> Self {
    let Vec4 {
      x: m0,
      y: m1,
      z: m2,
      w: m3,
    } = projection_matrix.get_col(0);
    let Vec4 {
      x: m4,
      y: m5,
      z: m6,
      w: m7,
    } = projection_matrix.get_col(1);
    let Vec4 {
      x: m8,
      y: m9,
      z: m10,
      w: m11,
    } = projection_matrix.get_col(2);
    let Vec4 {
      x: m12,
      y: m13,
      z: m14,
      w: m15,
    } = projection_matrix.get_col(3);
    let mut res = Self::default();
    res.planes[0].set_components(m3 - m0, m7 - m4, m11 - m8, m15 - m12);
    res.planes[1].set_components(m3 + m0, m7 + m4, m11 + m8, m15 + m12);
    res.planes[2].set_components(m3 + m1, m7 + m5, m11 + m9, m15 + m13);
    res.planes[3].set_components(m3 - m1, m7 - m5, m11 - m9, m15 - m13);
    res.planes[4].set_components(m3 - m2, m7 - m6, m11 - m10, m15 - m14);
    res.planes[5].set_components(m3 + m2, m7 + m6, m11 + m10, m15 + m14);

    res
  }

  pub fn intersect_object(&self, object: Rc<dyn IObject3D>) -> bool {}
}

use crate::math::{self, Mat4, Vec3};
pub struct Frustum {
  near: f32,
  far: f32,
  aspect: f32,
  fov: f32,
  mat: Mat4,
}

impl Frustum {
  #[rustfmt::skip]
  /**
   * fov => the angle from right/left plane to the camera direction
   */
  pub fn new(near: f32,far: f32, aspect: f32, fov: f32) -> Frustum {
    Self{
      near,
      aspect,
      fov,
      far,
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

pub struct Camera {
  frustum: Frustum,
  position: Vec3,
  rotation: Vec3,
  view_matarix: Mat4,
  view_direction: Vec3,
}

impl Camera {
  pub fn new(near: f32, far: f32, aspect: f32, fov: f32) -> Self {
    Self {
      frustum: Frustum::new(near, far, aspect, fov),
      position: Vec3::new(1.0, 1.0, 1.0),
      rotation: Vec3::zero(),
      view_matarix: Mat4::identity(),
      view_direction: -*Vec3::z_axis(),
    }
  }

  pub fn move_to(&mut self, position: Vec3) {
    self.position = position;
    self.compute_view_matrix();
  }

  pub fn move_delta(&mut self, delta: Vec3) {
    self.position += delta;
    self.compute_view_matrix();
  }

  pub fn set_rotation(&mut self, rotation: Vec3) {
    self.rotation = rotation;
    self.compute_view_matrix();
  }

  pub fn lookat(&mut self, point: Vec3) {
    // if location as (0,0,1) and look at 0,0,0, then direction is 0,0,-1
    // and reverse(back) is 0,0,1
    let back = (self.position - point).normalize();
    self.view_direction = -back;
    let dir = point - self.position;

    // 0,1,0
    let up = Vec3::y_axis();
    // -1,0,0
    let right = up.cross(&back).normalize();
    // 0,1,0
    let up = back.cross(&right).normalize();
  }

  pub fn compute_view_matrix(&mut self) {
    let rotation = math::apply_eular_rotate_xyz(&self.rotation);
    //SRT
    self.view_matarix = rotation * math::apply_translate(&self.position);
    // always compute from minus z axis
    self.view_direction = (rotation * math::Vec4::new(0.0, 0.0, -1.0, 1.0)).truncated_to_vec3();
  }

  pub fn get_frustum(&self) -> &Frustum {
    &self.frustum
  }

  pub fn get_position(&self) -> &Vec3 {
    &self.position
  }
  pub fn get_rotation(&self) -> &Vec3 {
    &self.rotation
  }
  pub fn get_view_matarix(&self) -> &Mat4 {
    &self.view_matarix
  }
  pub fn get_view_direction(&self) -> &Vec3 {
    &self.view_direction
  }
}

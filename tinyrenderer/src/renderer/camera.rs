use crate::math::{apply_eular_rotate_xyz, apply_translate, Mat4, Vec3, Vec4};

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
      mat: Mat4::from_row(&[
            near / half_w,           0.0,                       0.0,                               0.0,
                      0.0, near / half_h,                       0.0,                               0.0,
                      0.0,           0.0, (far + near) / (near - far), 2.0 * far * near / (near - far),
                      0.0,           0.0,                      -1.0,                               0.0,
          ])
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

pub struct Camera {
  frustum: Frustum,
  position: Vec3,
  rotation: Vec3,
  view_matrix: Mat4,
  view_direction: Vec3,
}

impl Camera {
  pub fn new(w: f32, h: f32) -> Self {
    Self {
      frustum: Frustum::new(1.0, 1000.0, w / h, 14f32.to_radians()),
      position: Vec3::zero(),
      rotation: Vec3::zero(),
      view_matrix: Mat4::identity(),
      view_direction: *Vec3::z_axis() * -1.0,
    }
  }

  pub fn get_view_matarix(&self) -> &Mat4 {
    &self.view_matrix
  }

  pub fn update_frustum(&mut self, near: f32, far: f32, fov: f32) {
    self.frustum = Frustum::new(near, far, self.frustum.aspect, fov);
  }

  fn compute_view_matrix(&mut self) {
    let rotation = apply_eular_rotate_xyz(&self.rotation);
    //SRT
    self.view_matrix = rotation * apply_translate(&self.position);
    // always compute from minus z axis
    self.view_direction = (rotation * Vec4::new(0.0, 0.0, -1.0, 1.0)).truncated_to_vec3();
  }

  pub fn move_to(&mut self, postion: Vec3) {
    self.position = postion;
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

  #[rustfmt::skip]
  pub fn lookat(&mut self, point: Vec3) {
    // the reverse for the looking at vector represents the equivalance of the +z
    let back = (self.position - point).normalize();
    self.view_direction =   back * -1.0;

    // 0,1,0
    let up = Vec3::y_axis();
    //  up x back = right basis
    let right = up.cross(&back).normalize();
    //  back x right= top basis
    let up = back.cross(&right).normalize();

    // Ro ==> Rc, B_Ro = [xa,ya,za] => B_Rc = [xa',ya',za'] = [right,up,back],  B_Rc^-1 = B_Rc^T = [right^T]
    //                   [xb,yb,zb]           [xb',yb',zb']                                        [  up^T ]
    //                   [xc,yc,zc]           [xc',yc',zc']                                        [ back^T]
    // posture = T · R · S · Standard   ===> Standard = S^-1 · R^-1 · T^-1 · posture
    // (R^-1 · T^-1) aka ViewMatrix
    // R^-1 =  B_Rc^-1 = B_Rc^T = [right.x, right.y, right.z,  0.0]   T^-1 = [1.0, 0.0, 0.0, -1 * position.x]
    //                            [   up.x,    up.y,    up.z,  0.0]          [0.0, 1.0, 0.0, -1 * position.y]
    //                            [ back.x,  back.y,  back.z,  0.0]          [0.0, 0.0, 1.0, -1 * position.z]
    //                            [    0.0,     0.0,     0.0,  1.0]          [0.0, 0.0, 0.0,             1.0]
    //
    // then R^-1 x T^-1  will have the form below
    self.view_matrix = Mat4::from_row(&[
      right.x, right.y, right.z, -right.dot(&self.position),
      up.x,    up.y,    up.z,    -up.dot(&self.position),
      back.x,  back.y,  back.z,  -back.dot(&self.position),
      0.0,     0.0,     0.0,                        1.0,
    ]);

    let looking_at = point - self.position;

    // calculate the angle to the rotating axis
    // a · b = ||a||·||b||·cosθ (if a & b normalized) ===> θ = arccose(a · b)
    // x is the rotating axis
    let angle_to_y = Vec3::y_axis().dot(&Vec3::new(0.0,looking_at.y, looking_at.z).normalize()).acos();
    // y is the rotating axis
    let angle_to_z = Vec3::z_axis().dot(&Vec3::new(looking_at.x, 0.0, looking_at.z).normalize()).acos();
    // z is the rotating axis
    let angle_to_x = Vec3::x_axis().dot(&Vec3::new(looking_at.x, looking_at.y, 0.0).normalize()).acos();

    self.view_direction = back * -1.0;

    self.rotation = Vec3::new(angle_to_y, angle_to_z, angle_to_x);
  }

  pub fn get_frustum(&self) -> &Frustum {
    &self.frustum
  }
}

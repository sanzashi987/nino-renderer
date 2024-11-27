use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  core::{object_3d::IObject3D, render_target::RenderTarget, uniform::Uniform},
  material::material::ToUniform,
  math::{Mat4, Vec2, Vec3, Vec4},
  objects::base::Object3D,
};

pub enum LightType {
  AmbientLight,
  DirectionalLight,
  LightProbe,
  PointLight,
  SpotLight,
  HemisphereLight,
  RectAreaLight,
}

pub trait ToUniformWithView {
  fn to_uniform(&self, camera: Rc<dyn ICamera>) -> Uniform;
}

pub trait ILight: IObject3D + ToUniformWithView + ToUniform {
  fn intensity(&self) -> f32;

  fn color(&self) -> Vec4;

  fn light_type(&self) -> LightType;

  fn shadow(&self) -> Option<Rc<dyn ILightShadow>>;

  fn target(&self) -> &Object3D;
}

#[rustfmt::skip]
static NDC_FACTOR: Mat4 = Mat4::from_row([			
  0.5, 0.0, 0.0, 0.5,
	0.0, 0.5, 0.0, 0.5,
	0.0, 0.0, 0.5, 0.5,
	0.0, 0.0, 0.0, 1.0
]);

pub trait ILightShadow: ILightShadowBase + ToUniform {
  fn update_matrices(&self, light: Rc<dyn ILight>, viewport: Vec4) {
    let global_light_position = light.global_matrix().get_col(3).truncated_to_vec3();
    self
      .camera()
      .update_from_global_position(global_light_position);
    let target_position = light
      .target()
      .global_matrix()
      .get_col(3)
      .truncated_to_vec3();

    self.camera().look_at(target_position);
    self.camera().update_global_matrix();

    let vp_matrix = self.camera().projection_matrix() * self.camera().view_matrix();

    self.set_matrix(NDC_FACTOR * vp_matrix);
  }
}
pub trait ILightShadowBase {
  fn set_matrix(&self, next: Mat4);

  fn matrix(&self) -> Mat4;

  fn camera(&self) -> Rc<dyn ICamera>;

  fn map_size(&self) -> Vec2;

  fn viewports(&self) -> &Vec<Vec4>;

  fn map(&self) -> Rc<RenderTarget>;
}

pub fn compute_direction(position: Vec3, target: Vec3, view_matrix: Mat4) -> Vec3 {
  let Vec3 { x, y, z } = position - target;
  let mut direction = Vec3::zero();
  direction.x = view_matrix.get(0, 0) * x + view_matrix.get(1, 0) * y + view_matrix.get(2, 0) * z;
  direction.y = view_matrix.get(0, 1) * x + view_matrix.get(1, 1) * y + view_matrix.get(2, 1) * z;
  direction.z = view_matrix.get(0, 2) * x + view_matrix.get(1, 2) * y + view_matrix.get(2, 2) * z;
  direction
}

macro_rules! init_shadow_map {
  ($camera:ident;$($val:ident),*) => {{
    Self{
      $($val,)*
      camera:$camera,
      intensity:1.0,
      bias: 0.0,
      normal_bias: 0.0,
      radius: 1.0,
      map_size: crate::math::Vec2::new(512.0,512.0),
      mat:  Default::default(),
      viewports: vec![crate::math::Vec4::new(0.0,0.0,1.0,1.0)],
      map:  Default::default(),
      matrix: Default::default(),
    }
  }};
}

pub(crate) use init_shadow_map;

use std::rc::Rc;

use renderer_macro_derive::object_3d;

use crate::{
  cameras::camera::ICamera,
  core::{
    object_3d::{with_default_fields, IObject3D},
    uniform::Uniform,
  },
  math::Vec4,
  objects::base::Object3D,
};

use super::light::{compute_direction, ILight, LightToUniform};

#[object_3d(IObject3D)]
pub struct DirectionalLight {
  pub color: Vec4,
  pub intensity: f32,
  pub target: Object3D,
  // pub shadow
  // pub shadow:Diresh
}

impl DirectionalLight {
  pub fn new() -> std::rc::Rc<Self> {
    let color = Vec4::default();
    let intensity = 1.0f32;
    let target = Object3D::new_ownership();
    let shadow = None;
    let this = with_default_fields!(Light;color,intensity,target,shadow);
    this
  }
}

impl LightToUniform for DirectionalLight {
  fn to_uniform(&self, camera: Rc<dyn ICamera>) -> Uniform {
    let mut res = Uniform::default();
    let rgb = self.color.truncated_to_vec3() * self.intensity;
    let color = Vec4::from_vec3(&rgb, self.color.w);

    let position = self.global_matrix().get_col(3).truncated_to_vec3();
    let target = self.target.global_matrix().get_col(3).truncated_to_vec3();
    let view_matrix = camera.view_matrix();

    let direction = compute_direction(position, target, view_matrix);

    res.insert("color", color);
    res.insert("direction", direction);

    res
  }
}

impl ILight for DirectionalLight {
  fn to_shadow_uniform(&self) -> crate::core::uniform::Uniform {
    todo!()
  }

  fn shadow(&self) -> Option<Rc<dyn super::light::ILightShadow>> {
    todo!()
  }

  fn light_type(&self) -> super::light::LightType {
    super::light::LightType::DirectionalLight
  }

  fn intensity(&self) -> f32 {
    self.intensity
  }

  fn color(&self) -> Vec4 {
    self.color
  }

  fn target(&self) -> &Object3D {
    todo!()
  }
}

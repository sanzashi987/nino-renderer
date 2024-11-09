use std::rc::Rc;

use renderer_macro_derive::object_3d;

use crate::{
  cameras::camera::{self, ICamera},
  core::{
    object_3d::{with_default_fields, IObject3D},
    uniform::Uniform,
  },
  math::Vec4,
  objects::base::Object3D,
};

use super::light::{ILight, LightToUniform};

#[object_3d(IObject3D)]
pub struct DirectionalLight {
  pub color: Vec4,
  pub intensity: f32,
  pub target: Object3D,
  // pub shadow
}

impl DirectionalLight {
  pub fn new() -> std::rc::Rc<Self> {
    let color = Vec4::default();
    let intensity = 1.0f32;
    let target = Object3D::new_ownership();
    let this = with_default_fields!(Light;color,intensity,target);
    this
  }
}

impl LightToUniform for DirectionalLight {
  fn to_uniform(&self, camera: Rc<dyn ICamera>) -> Uniform {
    let mut res = Uniform::default();
    let color = self.color * self.intensity;

    let position = self.global_matrix().get_col(3).truncated_to_vec3();
    let target_pos = self.target.global_matrix().get_col(3).truncated_to_vec3();

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
}

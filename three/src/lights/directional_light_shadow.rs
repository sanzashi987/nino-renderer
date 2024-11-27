use renderer_macro_derive::light_shadow;

use crate::{cameras::orthographic_camera::OrthographicCamera, material::material::ToUniform};

use super::light::{init_shadow_map, ILightShadow, ILightShadowBase};

#[light_shadow(ILightShadowBase)]
pub struct DirectionalLightShadow {}

impl ILightShadow for DirectionalLightShadow {}

impl DirectionalLightShadow {
  pub fn new() -> Self {
    let camera = OrthographicCamera::default();
    init_shadow_map!(camera;)
  }
}

impl ToUniform for DirectionalLightShadow {
  fn to_uniform(&self) -> crate::core::uniform::Uniform {
    todo!()
  }
}

use renderer_macro_derive::light_shadow;

use crate::{
  cameras::orthographic_camera::OrthographicCamera, core::uniform::Uniform,
  material::material::ToUniform,
};

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
    let mut res = Uniform::default();

    res.insert("shadowIntensity", self.intensity);
    res.insert("shadowBias", self.bias);
    res.insert("shadowNormalBias", self.normal_bias);
    res.insert("shadowRadius", self.radius);
    res.insert("shadowMap_size", self.map_size);

    res
  }
}

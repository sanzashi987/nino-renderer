use renderer_macro_derive::light_shadow;

use crate::cameras::orthographic_camera::OrthographicCamera;

use super::light::{ILightShadow, ILightShadowBase};

#[light_shadow(ILightShadowBase)]
pub struct DirectionalLightShadow {}

impl ILightShadow for DirectionalLightShadow {}

impl DirectionalLightShadow {
  pub fn new() -> Self {
    let camera = OrthographicCamera::new(-1.0, 1.0, 1.0, -1.0, 0.1, 2000.0);
    

  }
}

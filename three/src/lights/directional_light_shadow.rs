use renderer_macro_derive::light_shadow;

use super::light::{ILightShadow, ILightShadowBase};

#[light_shadow(ILightShadowBase)]
pub struct DirectionalLightShadow {}

impl ILightShadow for DirectionalLightShadow {}

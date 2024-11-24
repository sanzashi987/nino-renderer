use std::rc::Rc;

use crate::{
  cameras::camera::{self, ICamera},
  core::uniform::Uniform,
  lights::light::{ILight, LightType},
  math::{Mat4, Vec3, Vec4},
  textures::texture::Texture,
};

#[derive(Default)]
pub struct GLLights {
  pub ambient: Vec4,

  pub directional: Vec<Uniform>,
  pub directional_shadow: Vec<Uniform>,
  pub directional_shadow_map: Vec<Option<Texture>>,
  pub directional_shadow_matrix: Vec<Mat4>,

  pub point: Vec<Uniform>,
  pub point_shadow: Vec<Uniform>,
  pub point_shadow_map: Vec<Option<Texture>>,
  pub point_shadow_matrix: Vec<Mat4>,

  pub spot: Vec<Uniform>,
  pub spot_shadow: Vec<Uniform>,
  pub spot_shadow_map: Vec<Option<Texture>>,
  pub spot_shadow_matrix: Vec<Mat4>,
}

impl GLLights {
  pub fn setup(&mut self, lights: &Vec<Rc<dyn ILight>>) {
    for light in lights.iter() {
      match light.light_type() {
        LightType::AmbientLight => {
          let color = light.color();
          self.ambient.x += color.x;
          self.ambient.y += color.y;
          self.ambient.z += color.z;
        }
        LightType::DirectionalLight => {}
        LightType::SpotLight => todo!(),
        LightType::PointLight => todo!(),

        LightType::LightProbe => todo!(),
        LightType::HemisphereLight => todo!(),
        LightType::RectAreaLight => todo!(),
      }
    }
  }

  pub fn setup_view(&mut self, lights: &Vec<Rc<dyn ILight>>, camera: Rc<dyn ICamera>) {}
}

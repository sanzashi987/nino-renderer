use std::rc::Rc;

use crate::{
  cameras::camera::{self, ICamera},
  core::{render_target::RenderTarget, uniform::Uniform},
  lights::light::{ILight, LightType, ToUniformWithView},
  material::material::ToUniform,
  math::{Mat4, Vec3, Vec4},
};

#[derive(Debug, Default)]
pub struct LightQueue {
  uniform: Vec<Uniform>,
  shadow_uniform: Vec<Uniform>,
  shadow_map: Vec<Rc<RenderTarget>>,
  shadow_matrix: Vec<Mat4>,
}

impl LightQueue {
  pub fn clear(&mut self) {
    self.uniform.clear();
    self.shadow_uniform.clear();
    self.shadow_map.clear();
    self.shadow_matrix.clear();
  }
}

#[derive(Default)]
pub struct GLLights {
  pub ambient: Vec4,

  pub directional: LightQueue,
  pub point: LightQueue,
  pub spot: LightQueue,
}

impl GLLights {
  pub fn reset(&mut self) {
    self.ambient = Vec4::default();
    self.directional.clear();
    self.point.clear();
    self.spot.clear();
  }

  pub fn setup(&mut self, lights: &Vec<Rc<dyn ILight>>) {
    for light in lights.iter() {
      match light.light_type() {
        LightType::AmbientLight => {
          let color = light.color();
          self.ambient.x += color.x;
          self.ambient.y += color.y;
          self.ambient.z += color.z;
        }
        LightType::DirectionalLight => {
          let l = light.clone();
          let uniform = (l as Rc<dyn ToUniform>).to_uniform();
          self.directional.uniform.push(uniform);
          if light.cast_shadow() {
            if let Some(shadow) = light.shadow() {
              let shadow_uniform = shadow.to_uniform();
              self.directional.shadow_uniform.push(shadow_uniform);
              self.directional.shadow_map.push(shadow.map().clone());
              self.directional.shadow_matrix.push(shadow.matrix());
            }
          }
        }
        LightType::SpotLight => todo!(),
        LightType::PointLight => todo!(),

        LightType::LightProbe => todo!(),
        LightType::HemisphereLight => todo!(),
        LightType::RectAreaLight => todo!(),
      }
    }
  }

  pub fn setup_view(&mut self, lights: &Vec<Rc<dyn ILight>>, camera: Rc<dyn ICamera>) {
    let mut i = 0;
    for light in lights.iter() {
      match light.light_type() {
        LightType::DirectionalLight => {
          let dummy = Uniform::default();
          let uniform = std::mem::replace(&mut self.directional.uniform[i], dummy);
          let l = light.clone();
          let view_uniform = (l as Rc<dyn ToUniformWithView>).to_uniform(camera.clone());
          let view_uniform = view_uniform.merge(uniform);
          let _ = std::mem::replace(&mut self.directional.uniform[i], view_uniform);
        }
        LightType::SpotLight => todo!(),
        LightType::PointLight => todo!(),

        LightType::LightProbe => todo!(),
        LightType::HemisphereLight => todo!(),
        LightType::RectAreaLight => todo!(),
        LightType::AmbientLight => {}
      }
      i += 1;
    }
  }
}

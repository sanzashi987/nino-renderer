use std::{borrow::Borrow, rc::Rc};

use crate::core::render_target::RenderTarget;
use crate::core::viewport;
use crate::math::Vec4;
use crate::{
  cameras::camera::ICamera,
  core::object_3d::{IObject3D, ObjectType},
  lights::light::ILight,
  math::Vec2,
  objects::{base::Renderable, line::Line, mesh::Mesh, point::Point, scene::Scene},
};

macro_rules! rc_convert {
  ($source:tt;$($type:tt),+;$msg:tt) => {
    $(
      if let Ok(res) = Rc::downcast::<$type>($source.clone()) {
        res
      } else
    )+ {
      panic!($msg)
    }
  };
}
#[derive(Debug)]
enum ShadowMapType {
  BasicShadowMap,
  PCFShadowMap,
  PCFSoftShadowMap,
  VSMShadowMap,
}

impl Default for ShadowMapType {
  fn default() -> Self {
    Self::PCFShadowMap
  }
}

#[derive(Debug, Default)]
pub struct ShadowMap {
  enable: bool,
  shadow_type: ShadowMapType,
}

impl ShadowMap {
  pub fn render(&self, lights: &Vec<Rc<dyn ILight>>, scene: Rc<Scene>, camera: Rc<dyn ICamera>) {
    if !self.enable {
      return;
    }

    for light in lights {
      if let Some(shadow) = light.shadow() {
        let Vec2 {
          x: map_width,
          y: map_height,
        } = shadow.map_size();

        let map = shadow.map();

        map.update_texture_name(light.name() + ".shadow_map");
        shadow.camera().projection_matrix();

        let vps = shadow.viewports();
        for vp in vps {
          let mut viewport = Vec4::zero();
          //offset_x
          viewport.x = vp.x * map_width;
          //offset_y
          viewport.y = vp.y * map_height;
          //texture_width
          viewport.z = vp.z * map_width;
          //texture_height
          viewport.w = vp.w * map_height;
          shadow.update_matrices(light.clone(), viewport);
        }
      }
    }
  }

  fn render_object(
    &self,
    target: &RenderTarget,
    object: Rc<dyn IObject3D>,
    camera: Rc<dyn ICamera>,
    shadow_camera: Rc<dyn ICamera>,
    light: Rc<dyn ILight>,
  ) {
    let visible = object.layers().test(&camera.layers());

    match object.get_type() {
      ObjectType::Mesh | ObjectType::Line | ObjectType::Point => {
        let obj = object.clone();
        let renderable: Rc<dyn Renderable> =
          rc_convert!(obj;Mesh,Line,Point;"Unexpected Renderable Type");
        if !renderable.material().visible() {
          return;
        }
      }
      _ => {
        let children = object.children();

        for child in children.iter() {
          self.render_object(
            target,
            child.clone(),
            camera.clone(),
            shadow_camera.clone(),
            light.clone(),
          );
        }
      }
    }
  }

  fn redenr_to_target(&self, target: &RenderTarget) {}
}

pub(crate) use rc_convert;

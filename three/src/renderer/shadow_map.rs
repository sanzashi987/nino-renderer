use std::rc::Rc;

use crate::{
  cameras::camera::ICamera,
  core::object_3d::{IObject3D, ObjectType},
  lights::light::ILight,
  math::Vec2,
  objects::{
    base::{Object3D, Renderable},
    line::Line,
    mesh::Mesh,
    point::Point,
    scene::Scene,
  },
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

#[derive(Debug, Default)]
pub struct ShadowMap {
  enable: bool,
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

        let vps = shadow.viewports();
        for vp in vps {
          let offset_x = vp.x * map_width;
          let offset_y = vp.y * map_height;
          let texture_width = vp.z * map_width;
          let texture_height = vp.w * map_height;
        }
      }
    }
  }

  fn render_object(
    &self,
    object: Rc<dyn IObject3D>,
    camera: Rc<dyn ICamera>,
    shadow_camera: Rc<dyn ICamera>,
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

        for child in children {}
      }
    }
  }
}

pub(crate) use rc_convert;

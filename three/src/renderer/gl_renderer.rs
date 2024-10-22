use std::{borrow::Borrow, rc::Rc};

use super::super::objects::scene::Scene;
use super::viewport::Viewport;
use super::{super::cameras::camera::Camera, render_states::RenderState};
use crate::{
  core::object_3d::{ObjectActions, ObjectType},
  math::{
    data_array::{ColorBuffer, DepthBuffer},
    extract_normal_matrix,
  },
};
#[derive(Default)]
pub struct GlRenderer {
  viewport: Viewport,
  color: ColorBuffer,
  depth: DepthBuffer,
  shadow_map: bool,
  render_states: RenderState,
}

impl GlRenderer {
  pub fn new() -> Self {
    Self {
      viewport: Default::default(),
      color: Default::default(),
      depth: Default::default(),
      shadow_map: Default::default(),
      render_states: Default::default(),
    }
  }

  pub fn set_size(&mut self, w: f32, h: f32) {
    self.viewport.set_size(w, h);
    self.color = ColorBuffer::new(w as u32, h as u32);
    self.depth = DepthBuffer::new(w as u32, h as u32);
  }

  // pub fn set_pixel_ratio(&mut self, r: f32) {}

  fn take_color(&mut self) -> ColorBuffer {
    let w = self.color.width();
    let h = self.color.height();
    self.depth.clear(std::f32::MAX);
    std::mem::replace(&mut self.color, ColorBuffer::new(w, h))
  }

  pub fn render(&mut self, scene: Scene, camera: impl Camera + ObjectActions) -> ColorBuffer {
    scene.update_global_matrix();
    camera.update_global_matrix();

    let project_screen_matrix = camera.projection_matrix() * camera.global_matrix_inverse();
    // let frustum =

    self.take_color()
  }

  fn parse_object(
    &mut self,
    object: Rc<dyn ObjectActions>,
    camera: &(impl Camera + ObjectActions),
    group_order: i32,
    sort: bool,
  ) {
    if !object.visible() {
      return;
    }

    let visible = object.test_layers(&camera.layers());
    if visible {
      let object_type = object.get_type();

      match object.get_type() {
        ObjectType::Light => {
          self.render_states.push_light(object.clone());
        }
        ObjectType::Scene => todo!(),
        ObjectType::Object3D => todo!(),
        ObjectType::Camera => todo!(),
        ObjectType::Group => {}
        ObjectType::Mesh | ObjectType::Line | ObjectType::Point => {}
      }
    }

    let children = object.children();

    for child in children.iter() {
      self.parse_object(child.clone(), camera, group_order, sort);
    }
  }
}

fn render_objects(object: Rc<dyn ObjectActions>, camera: impl Camera + ObjectActions) {}

fn render_object(
  object: Rc<dyn ObjectActions>,
  scene: &Scene,
  camera: impl Camera + ObjectActions,
) {
  let mv = camera.global_matrix() * object.global_matrix();
  let normal_matrix = extract_normal_matrix(mv);
}

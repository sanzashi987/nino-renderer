use std::rc::Rc;

use super::super::cameras::camera::ICamera;
use super::super::objects::scene::Scene;
use super::render_states::{RenderList, RenderState, RenderStates};
use super::render_target::RenderTarget;
use crate::{
  core::object_3d::{ObjectActions, ObjectType},
  math::{
    data_array::{ColorBuffer, DepthBuffer},
    extract_normal_matrix,
  },
};
#[derive(Default)]
pub struct GlRenderer {
  result: RenderTarget,
  depth: DepthBuffer,
  shadow_map: bool,
  render_states: RenderStates,
  render_lists: RenderList,
  render_target: Option<RenderTarget>,
  current_render_state: Option<RenderState>,
}

impl GlRenderer {
  pub fn new() -> Self {
    Default::default()
  }

  // pub fn set_pixel_ratio(&mut self, r: f32) {}

  pub fn get_current_target(&self) -> &RenderTarget {
    if let Some(target) = &self.render_target {
      target
    } else {
      &self.result
    }
  }

  pub fn set_render_target(&mut self) {}

  pub fn set_size(&mut self, w: f32, h: f32) {
    self.result.set_size(w, h);
  }

  pub fn clear(&mut self) {
    self.depth.clear(std::f32::MAX);
  }

  pub fn render(&mut self, scene: Rc<Scene>, camera: Rc<dyn ICamera>) -> ColorBuffer {
    scene.update_global_matrix();
    camera.update_global_matrix();

    let project_matrix = camera.projection_matrix();
    let view_matrix = camera.global_matrix_inverse();

    let vp_matrix = project_matrix * view_matrix;

    let current_render_state = self.render_states.get(&scene.uuid());

    // let frustum =
    self.result.take_color()
  }

  fn project_object(
    &mut self,
    current_render_state: &RenderState,
    object: Rc<dyn ObjectActions>,
    camera: Rc<dyn ICamera>,
    group_order: i32,
    sort: bool,
  ) {
    if !object.visible() {
      return;
    }

    // let

    let visible = object.test_layers(&camera.layers());
    if visible {
      match object.get_type() {
        ObjectType::Light => {
          current_render_state.push_light(object.clone());

          if object.cast_shadow() {
            current_render_state.push_shadow(object.clone());
          }
        }
        ObjectType::Mesh | ObjectType::Line | ObjectType::Point => {
          
        }
        _ => {}
      }
    }

    let children = object.children();

    for child in children.iter() {
      self.project_object(
        current_render_state,
        child.clone(),
        camera.clone(),
        group_order,
        sort,
      );
    }
  }

  fn render_pixel(&mut self) {}
}

fn render_objects(object: Rc<dyn ObjectActions>, camera: Rc<dyn ICamera>) {}

fn render_object(object: Rc<dyn ObjectActions>, scene: &Scene, camera: Rc<dyn ICamera>) {
  let mv = camera.global_matrix() * object.global_matrix();
  let normal_matrix = extract_normal_matrix(mv);
}

use std::rc::Rc;

use super::super::cameras::camera::ICamera;
use super::super::objects::scene::Scene;
use super::render_states::{RenderItem, RenderList, RenderLists, RenderState, RenderStates};
use super::render_target::RenderTarget;
use super::shadow_map::ShadowMap;
use crate::core::buffer_geometry::IGeometry;
use crate::lights::directional_light::DirectionalLight;
use crate::lights::light::ILight;
use crate::material::material::IMaterial;
use crate::math::Mat4;
use crate::objects::base::Renderable;
use crate::objects::group::Group;
use crate::objects::line::Line;
use crate::objects::mesh::Mesh;
use crate::objects::point::Point;
use crate::{
  core::object_3d::{IObject3D, ObjectType},
  math::{
    data_array::{ColorBuffer, DepthBuffer},
    extract_normal_matrix,
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

#[derive(Default)]
pub struct GlRenderer {
  result: RenderTarget,
  depth: DepthBuffer,
  shadow_map: ShadowMap,
  render_states: RenderStates,
  render_lists: RenderLists,
  render_target: Option<RenderTarget>,
  current_render_state: Option<RenderState>,
  current_render_list: Option<RenderList>,
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

    let scene_id = scene.uuid();
    let current_render_state = { self.render_states.get(scene_id) };
    let current_render_list = self.render_lists.get(scene_id);

    // let frustum =

    project_object(
      current_render_state,
      current_render_list,
      scene.clone(),
      camera.clone(),
      vp_matrix,
      0,
      true,
    );

    current_render_list.finish();

    render_scene(current_render_list, scene.clone(), camera.clone());

    self.result.take_color()
  }
}

fn render_scene(render_list: &RenderList, scene: Rc<Scene>, camera: Rc<dyn ICamera>) {
  let opaque = render_list.opaque.borrow();
  if opaque.len() > 0 {
    render_objects(&opaque, scene.clone(), camera.clone());
  }
  let transmissive = render_list.transmissive.borrow();
  if transmissive.len() > 0 {
    render_objects(&transmissive, scene.clone(), camera.clone());
  }
  let transparent = render_list.transparent.borrow();
  if transparent.len() > 0 {
    render_objects(&transparent, scene.clone(), camera.clone());
  }
}

fn render_objects(render_items: &Vec<Rc<RenderItem>>, scene: Rc<Scene>, camera: Rc<dyn ICamera>) {
  for render_item in render_items {
    let object = render_item.object.clone();
    let geometry = render_item.geometry.clone();
    let material = render_item.material.clone();
    let parent = render_item.parent.clone();
    if object.layers().test(&camera.layers()) {
      let scene = scene.clone();
      let camera = camera.clone();
      render_object(object, scene, camera, geometry, material, parent);
    }
  }
}

fn render_object(
  object: Rc<dyn IObject3D>,
  scene: Rc<Scene>,
  camera: Rc<dyn ICamera>,
  geometry: Rc<dyn IGeometry>,
  material: Rc<dyn IMaterial>,
  parent: Option<Rc<dyn IObject3D>>,
) {
  let mv = camera.global_matrix() * object.global_matrix();
  let normal_matrix = extract_normal_matrix(mv);
}

fn project_object(
  current_render_state: &RenderState,
  current_render_list: &RenderList,
  object: Rc<dyn IObject3D>,
  camera: Rc<dyn ICamera>,
  vp: Mat4,
  group_order: i32,
  sort: bool,
) {
  if !object.visible() {
    return;
  }

  // let

  let visible = object.test_layers(&camera.layers());
  let mut next_group_order = group_order;
  if visible {
    match object.get_type() {
      ObjectType::Group => {
        if let Ok(res) = Rc::downcast::<Group>(object.clone()) {
          next_group_order = res.group_order;
        }
      }

      ObjectType::Light => {
        let obj = object.clone();
        let light: Rc<dyn ILight> = rc_convert!(obj;DirectionalLight;"Unexpected Light Type");
        current_render_state.push_light(light.clone());

        if object.cast_shadow() {
          current_render_state.push_shadow(light.clone());
        }
      }

      ObjectType::Mesh | ObjectType::Line | ObjectType::Point => {
        let obj = object.clone();
        let global_model = obj.global_matrix();

        let renderable: Rc<dyn Renderable> =
          rc_convert!(obj;Mesh,Line,Point;"Unexpected Renderable Type");

        let geometry = renderable.geometry();
        let material = renderable.material();

        let vec4 = vp * global_model.get_col(3);

        current_render_list.push(obj, geometry, material, next_group_order, vec4.z, None);
      }

      _ => {}
    }
  }

  let children = object.children();

  for child in children.iter() {
    project_object(
      current_render_state,
      current_render_list,
      child.clone(),
      camera.clone(),
      vp,
      next_group_order,
      sort,
    );
  }
}

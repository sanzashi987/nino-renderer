use std::rc::Rc;

use super::super::cameras::camera::ICamera;
use super::super::objects::scene::Scene;
use super::render_states::{RenderItem, RenderList, RenderLists, RenderState, RenderStates};
use super::shadow_map::ShadowMap;
use crate::core::buffer_geometry::IGeometry;
use crate::core::render_target::RenderTarget;
use crate::core::uniform::Uniform;
use crate::lights::directional_light::DirectionalLight;
use crate::lights::light::ILight;
use crate::material::material::IMaterial;
use crate::math::Mat4;
use crate::objects::base::Renderable;
use crate::objects::group::Group;
use crate::objects::line::Line;
use crate::objects::mesh::Mesh;
use crate::objects::point::Point;

use crate::utils::rc_convert;
use crate::{
  core::object_3d::{IObject3D, ObjectType},
  math::{
    data_array::{ColorBuffer, DepthBuffer},
    extract_normal_matrix,
  },
};

#[derive(Default)]
pub struct GlRenderer {
  result: RenderTarget,
  depth: DepthBuffer,
  shadow_map: ShadowMap,
  render_states: RenderStates,
  render_lists: RenderLists,
  render_target: Option<RenderTarget>,
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

  fn project_object(
    &self,
    current_render_list: Rc<RenderList>,
    current_render_state: Rc<RenderState>,
    object: Rc<dyn IObject3D>,
    camera: Rc<dyn ICamera>,
    vp: Mat4,
    group_order: i32,
    sort: bool,
  ) {
    if !object.visible() {
      return;
    }

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

      let children = object.children();

      for child in children.iter() {
        self.project_object(
          current_render_list.clone(),
          current_render_state.clone(),
          child.clone(),
          camera.clone(),
          vp,
          next_group_order,
          sort,
        );
      }
    }
  }

  pub fn render(&mut self, scene: Rc<Scene>, camera: Rc<dyn ICamera>) -> ColorBuffer {
    scene.update_global_matrix();
    camera.update_global_matrix();

    let project_matrix = camera.projection_matrix();
    let view_matrix = camera.global_matrix_inverse();
    let view_projection_matrix = project_matrix * view_matrix;

    let mut global_uniform = Uniform::default();
    global_uniform.insert("view_matrix", view_matrix);
    global_uniform.insert("project_matrix", project_matrix);
    global_uniform.insert("view_projection_matrix", view_projection_matrix);

    let scene_id = scene.uuid();
    let current_render_state = self.render_states.get(scene_id);
    // self.current_render_list =
    let current_render_list = self.render_lists.get(scene_id);

    self.project_object(
      current_render_list.clone(),
      current_render_state.clone(),
      scene.clone(),
      camera.clone(),
      view_projection_matrix,
      0,
      true,
    );

    current_render_list.finish();

    self.shadow_map.render(
      &current_render_state.shadows.borrow(),
      scene.clone(),
      camera.clone(),
    );

    current_render_state.setup_lights();
    self.render_scene(
      current_render_list.clone(),
      scene.clone(),
      camera.clone(),
      &mut global_uniform,
    );

    self.result.take_color()
  }

  fn render_scene(
    &self,
    render_list: Rc<RenderList>,
    scene: Rc<Scene>,
    camera: Rc<dyn ICamera>,
    global_uniform: &mut Uniform,
  ) {
    let opaque = render_list.opaque.borrow();
    if opaque.len() > 0 {
      self.render_objects(&opaque, scene.clone(), camera.clone(), global_uniform);
    }
    let transmissive = render_list.transmissive.borrow();
    if transmissive.len() > 0 {
      self.render_objects(&transmissive, scene.clone(), camera.clone(), global_uniform);
    }
    let transparent = render_list.transparent.borrow();
    if transparent.len() > 0 {
      self.render_objects(&transparent, scene.clone(), camera.clone(), global_uniform);
    }
  }

  fn render_objects(
    &self,
    render_items: &Vec<Rc<RenderItem>>,
    scene: Rc<Scene>,
    camera: Rc<dyn ICamera>,
    global_uniform: &mut Uniform,
  ) {
    for render_item in render_items {
      let object = render_item.object.clone();
      let geometry = render_item.geometry.clone();
      let material = render_item.material.clone();
      let parent = render_item.parent.clone();
      if object.layers().test(&camera.layers()) {
        let scene = scene.clone();
        let camera = camera.clone();
        self.render_object(
          object,
          scene,
          camera,
          geometry,
          material,
          parent,
          global_uniform,
        );
      }
    }
  }

  fn render_object(
    &self,
    object: Rc<dyn IObject3D>,
    scene: Rc<Scene>,
    camera: Rc<dyn ICamera>,
    geometry: Rc<dyn IGeometry>,
    material: Rc<dyn IMaterial>,
    parent: Option<Rc<dyn IObject3D>>,
    global_uniform: &mut Uniform,
  ) {
    let mut m_uniform = material.to_uniform();
    let model_matrix = object.global_matrix();
    let view_matrix = camera.view_matrix();
    let model_view_matrix = view_matrix * model_matrix;
    let normal_matrix = extract_normal_matrix(model_view_matrix);

    m_uniform.insert("model_matrix", model_matrix);
    m_uniform.insert("normal_matrix", normal_matrix);

    let mut uniform = global_uniform.merge(m_uniform);

    // uniform.insert("model_matrix", model_matrix);

    // let
  }
}

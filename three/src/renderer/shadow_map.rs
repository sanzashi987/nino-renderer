use std::rc::Rc;

use crate::core::render_target::RenderTarget;
use crate::material::depth_material::MeshDepthMaterial;
use crate::material::material::IMaterial;
use crate::math::data_array::DepthBuffer;
use crate::math::Vec4;
use crate::utils::rc_convert;
use crate::{
  cameras::camera::ICamera,
  core::object_3d::{IObject3D, ObjectType},
  lights::light::ILight,
  math::Vec2,
  objects::{base::Renderable, line::Line, mesh::Mesh, point::Point, scene::Scene},
};

use super::render_pipeline::render_pipeline;

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

#[derive(Default)]
pub struct ShadowMap {
  enable: bool,
  shadow_type: ShadowMapType,
  material: MeshDepthMaterial,
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
          self.render_object(
            shadow.map().clone(),
            scene.clone(),
            camera.clone(),
            shadow.camera().clone(),
            light.clone(),
          );
        }
      }
    }
  }

  fn render_object(
    &self,
    target: Rc<RenderTarget>,
    object: Rc<dyn IObject3D>,
    camera: Rc<dyn ICamera>,
    shadow_camera: Rc<dyn ICamera>,
    light: Rc<dyn ILight>,
  ) {
    if !object.layers().test(&camera.layers()) {
      return;
    }

    match object.get_type() {
      ObjectType::Mesh | ObjectType::Line | ObjectType::Point => {
        let obj = object.clone();
        let renderable: Rc<dyn Renderable> =
          rc_convert!(obj;Mesh,Line,Point;"Unexpected Renderable Type");
        if !renderable.material().visible() {
          return;
        }

        let material = renderable.material();
        let depth_material = to_depth_material(object.clone(), material, light.clone());
        let geometry = renderable.geometry();
        let mut depth_buffer = DepthBuffer::default();
        render_pipeline(
          &target,
          &mut depth_buffer,
          camera,
          object,
          geometry,
          depth_material,
          None,
        );
      }
      _ => {
        let children = object.children();

        for child in children.iter() {
          self.render_object(
            target.clone(),
            child.clone(),
            camera.clone(),
            shadow_camera.clone(),
            light.clone(),
          );
        }
      }
    }
  }
}

fn to_depth_material(
  object: Rc<dyn IObject3D>,
  material: Rc<dyn IMaterial>,
  light: Rc<dyn ILight>,
) -> Rc<dyn IMaterial> {
}

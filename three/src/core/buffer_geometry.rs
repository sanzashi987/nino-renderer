use super::{
  buffer_attribute::{IBufferAttribute, TypeBufferEnum},
  geometries::{Box3, Sphere},
};
use std::{borrow::Borrow, collections::HashMap};

pub struct BufferGeometry {
  attributes: Attribute,
  uuid: String,
  bounding_sphere: Sphere,
}

impl Default for BufferGeometry {
  fn default() -> Self {
    Self {
      attributes: Default::default(),
      uuid: uuid::Uuid::new_v4().to_string(),
      bounding_sphere: Default::default(),
    }
  }
}

pub type Attribute = HashMap<String, TypeBufferEnum>;

pub trait IGeometry {
  fn get_uuid(&self) -> &str;
  fn get_attribute(&self) -> &Attribute;
  fn set_attribute(&mut self, key: &str, val: TypeBufferEnum);
  fn update_bounding_sphere(&mut self);
  fn bounding_sphere(&self) -> &Sphere;
}

impl IGeometry for BufferGeometry {
  fn get_attribute(&self) -> &Attribute {
    // let res = self.attributes.get("").map_or(None, |v| v.extract());
    &self.attributes
  }

  fn set_attribute(&mut self, key: &str, val: TypeBufferEnum) {
    self.attributes.insert(key.to_string(), val);
  }

  fn get_uuid(&self) -> &str {
    &self.uuid
  }

  fn update_bounding_sphere(&mut self) {
    if let Some(e) = self.attributes.get("position") {
      if let TypeBufferEnum::F32(position) = e {
        let mut box3 = Box3::default();
        // let a: &Box<dyn IBufferAttribute<f32>> = position;
        box3.from_attribute::<f32>(position);

        let count = position.items();
        let center = self.bounding_sphere.center;
        let mut max_radius = 0f32;
        for i in 0..count {
          let attr_vec3 = position.get_vec3(i).distance_to(center);
          max_radius = max_radius.max(attr_vec3);
        }

        self.bounding_sphere.radius = max_radius
      }
    }
  }

  fn bounding_sphere(&self) -> &Sphere {
    todo!()
  }
}

macro_rules! attribute {
  ($attribute:ident,$type:ty,$key:tt) => {
    let res: Option<$type> = $attribute.get($key).map_or(None, |v| v.extract());
    res
  };

  ($attribute:ident, $type:ty, $key:tt, !) => {
    Extract::<$type>::extract(
      ($attribute
        .get($key)
        .expect(&format!("error from getting {} from attribute", $key))),
    )
    .expect(&format!(
      "errot from parsing attribute '{}' value to  type '{}'",
      $key,
      stringify!($type)
    ))
  };
}

pub(crate) use attribute;

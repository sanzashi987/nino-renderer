use std::collections::HashMap;
use super::buffer_attribute::TypeBufferEnum;

pub struct BufferGeometry {
  attributes: HashMap<String, TypeBufferEnum>,
}

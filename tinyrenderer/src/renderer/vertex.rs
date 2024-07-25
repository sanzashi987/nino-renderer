use crate::{math::Vec4, model::Material};

#[derive(Debug)]
pub struct GLVertex<'a> {
  pub position: Vec4,
  pub material: Material<'a>,
}

impl<'a> GLVertex<'a> {
  pub fn from_vertex() -> Self {}
}

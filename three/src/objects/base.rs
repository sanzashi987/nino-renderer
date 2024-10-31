use std::rc::Rc;

use crate::{core::buffer_geometry::IGeometry, material::material::IMaterial};

pub trait Renderable {
  fn geometry(&self) -> Rc<dyn IGeometry>;

  fn material(&self) -> Rc<dyn IMaterial>;
}

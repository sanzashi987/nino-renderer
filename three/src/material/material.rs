use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::core::{buffer_geometry::Attribute, unifrom::Uniform, varying::Varying};

use super::shader::{DefineShader, GlPerFragment, GlPerVertex, Shader};

#[derive(Debug)]
pub enum DepthFunc {
  NeverDepth,
  AlwaysDepth,
  LessDepth,
  LessEqualDepth,
  EqualDepth,
  GreaterEqualDepth,
  GreaterDepth,
  NotEqualDepth,
}

#[derive(Debug)]
pub enum Blending {
  NoBlending,
  NormalBlending,
  AdditiveBlending,
  SubtractiveBlending,
  MultiplyBlending,
  CustomBlending,
}

#[derive(Debug)]
pub enum Side {
  FrontSide,
  BackSide,
  DoubleSide,
}

impl Default for Side {
  fn default() -> Self {
    Self::FrontSide
  }
}

impl Default for Blending {
  fn default() -> Self {
    Self::NormalBlending
  }
}

impl Default for DepthFunc {
  fn default() -> Self {
    Self::LessEqualDepth
  }
}

pub trait ConvertUniform {
  fn to_uniform(&self) -> Uniform;
}

#[derive(Debug)]
pub struct BasicMaterial<T: ConvertUniform + Default, U: DefineShader> {
  pub user_data: HashMap<String, Rc<dyn Any>>,

  pub blending: Blending,
  pub side: Side,

  pub opacity: u8,
  pub transparent: bool,

  pub depth_test: bool,
  pub depth_func: DepthFunc,
  pub depth_write: bool,

  attributes: RefCell<Rc<T>>,

  abstract_shader: PhantomData<U>,
}

trait RunShader {
  fn vertex(&self, a: &Attribute, u: &Uniform, v: &mut Varying, gl: &mut GlPerVertex);
  fn fragment(&self, u: &Uniform, v: &Varying, gl: &mut GlPerFragment) -> bool;
}

impl<T: ConvertUniform + Default, U: DefineShader> ConvertUniform for BasicMaterial<T, U> {
  fn to_uniform(&self) -> Uniform {
    self.attributes.borrow().to_uniform()
  }
}

impl<T: ConvertUniform + Default, U: DefineShader> RunShader for BasicMaterial<T, U> {
  fn vertex(&self, a: &Attribute, u: &Uniform, v: &mut Varying, gl: &mut GlPerVertex) {
    U::vertex()(a, u, v, gl)
  }

  fn fragment(&self, u: &Uniform, v: &Varying, gl: &mut GlPerFragment) -> bool {
    U::fragment()(u, v, gl)
  }
}

pub trait MaterialActions: ConvertUniform + RunShader {}

impl<T: ConvertUniform + Default, U: DefineShader> Default for BasicMaterial<T, U> {
  fn default() -> Self {
    Self {
      user_data: Default::default(),
      blending: Default::default(),
      side: Default::default(),
      opacity: Default::default(),
      transparent: Default::default(),
      depth_test: Default::default(),
      depth_func: Default::default(),
      depth_write: Default::default(),
      attributes: Default::default(),
      abstract_shader: Default::default(),
    }
  }
}

macro_rules! define_uniform_attr {
  ($uniform:ident;$($field:tt,)*) => {
    $(
      let Some(val) = self.$field {
        let e : crate::core::uniform::UnifromTypeEnum = val.into();
        uniform.attributes.insert(stringify!($field).to_string(), e);
      };
    )*

  };
}

pub(crate) use define_uniform_attr;

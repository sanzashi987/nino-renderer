use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::core::{buffer_geometry::Attribute, unifrom::Uniform, varying::Varying};

use super::shader::{DefineShader, GlPerFragment, GlPerVertex, Shader};

#[derive(Debug)]
enum DepthFunc {
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
enum Blending {
  NoBlending,
  NormalBlending,
  AdditiveBlending,
  SubtractiveBlending,
  MultiplyBlending,
  CustomBlending,
}

#[derive(Debug)]
enum Side {
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

#[derive(Debug, Default)]
pub struct BasicMaterial<T: ConvertUniform, U: DefineShader> {
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
  fn fragment(&self, u: &Uniform, v: &Varying, gl: &mut GlPerFragment);
}

impl<T: ConvertUniform, U: DefineShader> ConvertUniform for BasicMaterial<T, U> {
  fn to_uniform(&self) -> Uniform {
    self.attributes.borrow().to_uniform()
  }
}

impl<T: ConvertUniform, U: DefineShader> RunShader for BasicMaterial<T, U> {
  fn vertex(&self, a: &Attribute, u: &Uniform, v: &mut Varying, gl: &mut GlPerVertex) {
    U::vertex(&self)(a, u, v, gl)
  }

  fn fragment(&self, u: &Uniform, v: &Varying, gl: &mut GlPerFragment) {
    U::fragment(&self)(u, v, gl)
  }
}

pub trait MaterialActions: ConvertUniform + RunShader {}

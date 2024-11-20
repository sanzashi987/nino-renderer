use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::core::{buffer_geometry::Attribute, uniform::Uniform, varying::Varying};

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

pub trait ToUniform {
  fn to_uniform(&self) -> Uniform;
}

#[derive(Debug)]
pub struct BasicMaterial<T: ToUniform + Default, U: DefineShader> {
  pub user_data: HashMap<String, Rc<dyn Any>>,

  pub blending: Blending,
  pub side: Side,

  pub opacity: u8,
  pub transparent: bool,
  pub transmission: Option<u32>,
  pub visible: bool,

  pub depth_test: bool,
  pub depth_func: DepthFunc,
  pub depth_write: bool,

  pub wireframe: bool,
  pub wireframe_linewidth: u8,

  pub attributes: RefCell<Rc<T>>,

  pub abstract_shader: PhantomData<U>,
}

pub trait IMaterial: RunShader {
  fn transparent(&self) -> bool;
  fn transmission(&self) -> Option<u32>;
  fn visible(&self) -> bool;
  fn wireframe(&self) -> bool;
  fn wireframe_linewidth(&self) -> u8;
  fn to_uniform(&self) -> Uniform;
}

impl<T: ToUniform + Default, U: DefineShader> IMaterial for BasicMaterial<T, U> {
  fn transparent(&self) -> bool {
    self.transparent
  }
  fn transmission(&self) -> Option<u32> {
    self.transmission
  }
  fn visible(&self) -> bool {
    self.visible
  }

  fn to_uniform(&self) -> Uniform {
    self.attributes.borrow().to_uniform()
  }

  fn wireframe(&self) -> bool {
    self.wireframe
  }

  fn wireframe_linewidth(&self) -> u8 {
    self.wireframe_linewidth
  }
}

pub trait RunShader {
  fn vertex(&self, a: &Attribute, u: &Uniform, v: &mut Varying, gl: &mut GlPerVertex);
  fn fragment(&self, u: &Uniform, v: &Varying, gl: &mut GlPerFragment) -> bool;
}

impl<T: ToUniform + Default, U: DefineShader> RunShader for BasicMaterial<T, U> {
  fn vertex(&self, a: &Attribute, u: &Uniform, v: &mut Varying, gl: &mut GlPerVertex) {
    U::vertex()(a, u, v, gl)
  }

  fn fragment(&self, u: &Uniform, v: &Varying, gl: &mut GlPerFragment) -> bool {
    U::fragment()(u, v, gl)
  }
}

impl<T: ToUniform + Default, U: DefineShader> Default for BasicMaterial<T, U> {
  fn default() -> Self {
    Self {
      user_data: Default::default(),
      blending: Default::default(),
      side: Default::default(),
      opacity: Default::default(),
      transparent: Default::default(),
      transmission: Default::default(),
      visible: Default::default(),
      depth_test: Default::default(),
      depth_func: Default::default(),
      depth_write: Default::default(),
      attributes: Default::default(),
      abstract_shader: Default::default(),
      wireframe: Default::default(),
      wireframe_linewidth: Default::default(),
    }
  }
}

macro_rules! define_material_attribute {
  ($name:tt; $($key:tt->$field:tt:$type:ty),+) => {
#[derive(Debug,Default)]
pub struct $name {
  $(pub $field: Option<$type>,)+
}

impl crate::material::material::ToUniform for $name {
  fn to_uniform(&self) -> crate::core::uniform::Uniform {
    let mut uniform: crate::core::uniform::Uniform = Default::default();
    $(
      if let Some(val) = self.$field {
        let e : crate::core::uniform::UniformTypeEnum = val.into();
        uniform.insert(stringify!($field), e);
      };
    )+
    uniform
  }
}

impl From<&MtlData> for $name {
  fn from(value: &MtlData) -> Self {
    let mut res = Self::default();
    $(
      if let Some(v) = value.get_attr(stringify!($key)) {
        if let Some(matched) = v.downcast_ref::<$type>() {
          res.$field = Some(*matched);
        }
      }
    )+
    res
  }
}

  };
}

pub(crate) use define_material_attribute;

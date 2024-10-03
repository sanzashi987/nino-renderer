use std::{collections::HashMap, sync::Mutex};

use crate::math::{Vec2, Vec3};

use super::parser::Parser;

#[derive(Debug, Default, Clone, Copy)]
pub struct VertexIndex {
  pub position_index: u32,
  pub normal_index: Option<u32>,
  pub uv_index: Option<u32>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Face {
  pub vertices: [VertexIndex; 3],
}

pub struct ObjInfo {
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub uvs: Vec<Vec2>,
}

pub struct Model {
  name: String,
}

// impl ObjLoader {}

// static LOADED_OBJS: Mutex<HashMap<u32, Mesh>> = Mutex::new(HashMap::new());

type ObjData = Vec<Model>;

struct ObjParserImpl;
impl Parseline for ObjParserImpl {}

pub type ObjParser = Parser<ObjData, ObjParserImpl>;
#[derive(Debug, Default)]
struct ObjLoader {
  next_model_id: u32,
  loaded_objs: HashMap<u32, Model>,
  path_id_map: HashMap<String, u32>,
}

impl ObjLoader {
  pub fn load(&mut self, filepath: &str) {
    
  }
}
pub static mut obj_loader: Mutex<ObjLoader> = Mutex::new(Default::default());

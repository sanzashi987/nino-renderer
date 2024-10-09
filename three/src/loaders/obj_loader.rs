use std::path::PathBuf;

use lazy_static::lazy_static;

use crate::math::{Vec2, Vec3};

use super::{
  defines::{parse_num, parse_token, ParserError},
  parser::{AssignId, Loader, Parse},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct VertexIndex {
  pub position_index: u32,
  pub normal_index: Option<u32>,
  pub uv_index: Option<u32>,
}

impl VertexIndex {
  pub fn new(p: u32, n: Option<u32>, u: Option<u32>) -> Self {
    Self {
      position_index: p,
      normal_index: n,
      uv_index: u,
    }
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Face {
  pub vertices: [VertexIndex; 3],
}

#[derive(Debug, Default)]
pub struct Model {
  pub name: String,
  pub faces: Vec<Face>,
  pub material: Option<String>,
}
#[derive(Debug, Default)]
struct ObjData {
  pub uid: u32,
  pub path: String,
  pub models: Vec<Model>,
  pub vertices: Vec<Vec3>,
  pub normals: Vec<Vec3>,
  pub uvs: Vec<Vec2>,
  pub mtl_libs: Vec<String>,
}

impl AssignId for ObjData {
  fn assign_id(&mut self, id: u32) {
    self.uid = id;
  }
}

impl ObjData {
  pub fn new_model(&mut self, name: String) {
    let mut model = Model::default();
    model.name = name;
    self.models.push(model);
  }

  pub fn add_face(&mut self, face: Face) -> Result<(), ParserError> {
    self
      .models
      .last_mut()
      .ok_or(ParserError::ModelNotInit)?
      .faces
      .push(face);
    Ok(())
  }

  pub fn add_vertex(&mut self, vertex: Vec3) {
    self.vertices.push(vertex)
  }

  pub fn add_normal(&mut self, normal: Vec3) {
    self.normals.push(normal)
  }

  pub fn add_uv(&mut self, uv: Vec2) {
    self.uvs.push(uv)
  }

  pub fn bind_material(&mut self, name: String) -> Result<(), ParserError> {
    let mtllib: &String = self.mtl_libs.last().ok_or(ParserError::MtlNotFound)?;
    let model = self.models.last_mut().ok_or(ParserError::ModelNotInit)?;
    let scoped_name = format!("{}@{}", mtllib, &name);
    model.material = Some(scoped_name);
    Ok(())
  }

  pub fn add_mtllib(&mut self, working_dir: &str, name: String) -> Result<(), ParserError> {
    let mut mtl_path = PathBuf::from(working_dir);
    mtl_path.push(&name);
    let mtl_path = mtl_path
      .to_str()
      .ok_or(ParserError::ModelNotInit)?
      .to_string();

    self.mtl_libs.push(mtl_path);
    Ok(())
  }

  pub fn get_material_path<F: FnMut(&str)>(&self, mut f: F) {
    for s in &self.mtl_libs {
      f(s)
    }
  }
}

struct ObjParserImpl;
impl Parse<ObjData> for ObjParserImpl {
  fn parse_line(
    data: &mut ObjData,
    tokens: &mut std::str::SplitWhitespace,
    working_dir: &str,
    token_str: &str,
  ) -> super::defines::ParserResult {
    match token_str {
      "#" => {}
      "g" | "o" => data.new_model(parse_token!(tokens.next(); String)?),
      "v" => data.add_vertex(parse_token!(tokens.next(); Vec3 = x:f32, y:f32, z:f32)?),
      "vn" => data.add_normal(parse_token!(tokens.next(); Vec3 = x:f32, y:f32, z:f32)?),
      "vt" => data.add_uv(parse_token!(tokens.next(); Vec2 = x:f32, y:f32)?),
      "mtllib" => data.add_mtllib(working_dir, parse_token!(tokens.next(); String)?)?,
      "usemtl" => data.bind_material(parse_token!(tokens.next(); String)?)?,
      "f" => {
        let mut vertex_vec = vec![];
        let mut done = false;

        while !done {
          match tokens.next() {
            Some(str) => {
              let splited: Vec<&str> = str.split("/").collect();
              let indices = splited.as_slice();
              if indices.len() > 3 || indices.len() < 1 {
                return Err(ParserError::InvalidSyntax(format!("face vertex indices")));
              }

              let (mut texture_index, mut normal_index) = (None, None);

              match *indices {
                [_, second, third] => {
                  if !second.is_empty() {
                    texture_index = Some(parse_num!(second, u32) - 1);
                  }
                  normal_index = Some(parse_num!(third, u32) - 1)
                }
                [_, second] => {
                  texture_index = Some(parse_num!(second, u32) - 1);
                }
                _ => return Err(ParserError::InvalidSyntax("face vertex format".to_string())),
              }

              let vertex_index = parse_num!(indices[0], u32) - 1;
              vertex_vec.push(VertexIndex::new(vertex_index, normal_index, texture_index));
            }
            None => {
              done = true;
            }
          }
        }
        if vertex_vec.len() != 3 {
          return Err(ParserError::InvalidSyntax("Face Vertices".to_string()));
        }

        let vertices: [VertexIndex; 3] = [vertex_vec[0], vertex_vec[1], vertex_vec[2]];
        data.add_face(Face { vertices })?;
      }
      _ => {}
    }
    Ok(())
  }
}

type ObjLoader = Loader<ObjData, ObjParserImpl>;

lazy_static! {
  pub static ref obj_loader: ObjLoader = Default::default();
}

pub struct Vertex {
  pub vertex: u32,
  pub normal: Option<u32>,
  pub textcoord: Option<u32>,
}

pub struct Face {
  pub vertices: Vec<Vertex>,
}

pub struct Model {
  pub faces: Vec<Face>,
  pub name: String,
  pub mtllib: Option<u32>,
  pub material: Option<String>,
  pub smooth_shade: u8,
}

// mtl -> Material Template Library
use super::{
  file_loader::FileLoader,
  model::{Model, Scene, VertexPointer},
};

pub struct MtlParser<'a> {
  loader: FileLoader<'a>,
}

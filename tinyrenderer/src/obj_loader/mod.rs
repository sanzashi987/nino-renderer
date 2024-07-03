mod file_loader;
mod model;
mod obj_parser;
mod error;


pub use error::ParserError;
pub use obj_parser::{load_obj, ParserMode};

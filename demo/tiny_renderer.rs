const RESOURCE_PATH: &str = "./resources";
const FOLDER: &str = "african";
const MODEL: &str = "head.obj";

use tinyrenderer::obj_loader::{load_obj, ParserMode};

fn get_resource_filepath(relative: &str) -> String {
  format!("{}/{}/{}", RESOURCE_PATH, FOLDER, relative)
}
fn main() {
  let relative_path = get_resource_filepath(MODEL);

  let mut res = load_obj(&relative_path, ParserMode::Lazy).unwrap();

  let scene = res.get_result().unwrap();

  println!("{:?}", scene);
}

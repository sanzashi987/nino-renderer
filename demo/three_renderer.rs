use three::loaders::{self, obj_loader::obj_loader};

const PATH: &str = "./resources/Son Goku/Goku.obj";
fn main() {
  let mut loader = obj_loader.lock().unwrap();
  let res = loader.load(PATH);

  dbg!("{}", res);
}

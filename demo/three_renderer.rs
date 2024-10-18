use three::loaders::{self, obj_loader::obj_loader};

const PATH: &str = "./resources/Son Goku/Goku.obj";
fn main() {
  let res = obj_loader.lock().unwrap().load(PATH);

  dbg!("{}", res)
}

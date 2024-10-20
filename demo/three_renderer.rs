use three::{
  loaders::{mtl_loader::mtl_loader, obj_loader::obj_loader, texture_loader::texture_loader},
  utils::SingleOrList,
};

const PATH: &str = ".\\resources\\Son Goku\\Goku.obj";
fn main() {
  let mut loader = obj_loader.lock().unwrap();
  let res = loader.load(PATH).unwrap();
  let mut_mtl_loader = mtl_loader.lock().unwrap();
  let mut_lv_loader = texture_loader.lock().unwrap();

  if let SingleOrList::Data(d) = res {
    let data = mut_mtl_loader
      .get_by_fullpath(&(&d.models[0].material.as_ref().unwrap()))
      .unwrap();
    let uv = mut_lv_loader.get_by_id(0).unwrap();
    dbg!(&d.models[0].material, &data.textures, &uv.path, &uv.id);
  }
  // dbg!("{}", data);
  // dbg!("{}", res);
}

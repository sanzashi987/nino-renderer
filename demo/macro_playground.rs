use renderer_macro_derive::object_3d;

struct A {}
struct B {}
struct C {}

enum MyEnum {}
#[object_3d(MyEnum, Transform)]
struct Test {
  pub a: String,
  pub b: u32,
}

fn main() {
  // print!("{}", 2);
}

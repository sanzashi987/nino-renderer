use renderer_macro_derive::renderer;

struct A {}
struct B {}
struct C {}

#[renderer("A", "C")]
struct Test {
  a: String,
  b: u32,
}

fn main() {
  // print!("{}", 2);
  let a = Test{
    
  }

}

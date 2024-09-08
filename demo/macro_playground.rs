use std::any::Any;

// use renderer_macro_derive::object_3d;

struct A {}
struct B {}
struct C {}

trait Transform {}
enum MyEnum {}
// #[object_3d(MyEnum, Transform)]
struct Test {
  children: Vec<Box<dyn ITrait>>,
}
impl Test {
  pub fn add(&mut self, c: impl ITrait) {
    self.children.push(Box::new(c))
  }
}

trait ITrait {
  fn log(&self);
}

impl ITrait for A {
  fn log(&self) {
    println!("a")
  }
}

impl ITrait for B {
  fn log(&self) {
    println!("b")
  }
}
fn main() {
  // print!("{}", 2);
  let a = A {};
  let b = B {};
  let v = vec![1u32];

  let any: &dyn Any = &v;

  if let Some(v) = any.downcast_ref::<Vec<f32>>() {
    println!("a>>>")
  }

  let mut t = Test { children: vec![] };

  t.add(a);
  t.add(b);
}

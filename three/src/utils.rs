pub fn swap_and_move<T: Default>(val: &mut T) -> T {
  std::mem::replace(val, Default::default())
}
#[derive(Debug)]
pub enum SingleOrList<T> {
  Data(T),
  List(Vec<T>),
}

macro_rules! rc_convert {
  ($source:tt;$($type:tt),+;$msg:tt) => {
    $(
      if let Ok(res) = std::rc::Rc::downcast::<$type>($source.clone()) {
        res
      } else
    )+ {
      panic!($msg)
    }
  };
}

pub(crate) use rc_convert;

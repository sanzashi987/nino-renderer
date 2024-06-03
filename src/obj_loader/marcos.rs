// #[macro_export]
macro_rules! ignore_utils {
  ($var:ident; $op:expr ;$($cond:expr),+) => {
    while $($var !=$cond &&)+ true{
      $var = $op;
    }
  };
}

pub(in crate::obj_loader) use ignore_utils;

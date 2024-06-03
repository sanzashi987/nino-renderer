// #[macro_export]
macro_rules! ignore_utils {
  ($var:ident; $op:expr ;$($cond:expr),+) => {
    while $($var !=$cond &&)+ true{
      $var = $op;
    }
  };
}

macro_rules! parse_as {
  ($var:ident; $request:expr; $type:ty) => {
    {

      $var = $request;
      let result  = if let TokenType::Token(content) = $var {
        Ok(content.parse::<$type>().map_err(|_| Error::CantConvertToNum))
      } else {
        Err(Error::ParseIncomplete)
      }
      //
      $var = $request;
      result
    }

  };
}

pub(in crate::obj_loader) use ignore_utils;
pub(in crate::obj_loader) use parse_as;

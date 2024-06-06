// #[macro_export]
macro_rules! skip_to_next_line {
  ($var:ident = $op:expr ;$($cond:expr),+) => {
    while $($var !=$cond &&)+ true{
      $var = $op;
    }
  };
}

macro_rules! parse_num {
  ($var:ident,$type:ty) => {{
    $var.parse::<$type>().map_err(|_| Error::CantConvertToNum)?
  }};
}

macro_rules! parse_line {
  ($var:ident = $request:expr; $type:ty) => {{
    //get current token
    $var = $request;
    let result = if let TokenType::Token(content) = $var {
      content
        .parse::<$type>()
        .map_err(|_| Error::CantConvertToNum)
    } else {
      Err(Error::ParseIncomplete)
    };
    // move to next token
    $var = $request;
    result
  }};

  ($var:ident = $request:expr; $type:ty = $($attr:ident : $attr_type:ty),+) => {
    {
      let mut value = <$type>::zero();
      $(
        //get current token
        $var = $request;
        if let TokenType::Token(num_str) = $var {
          // value.$attr  = num_str.parse::<$attr_type>().map_err(|_| Error::CantConvertToNum)?
          value.$attr = parse_num!(num_str,$attr_type)
          //  num_str.parse::<$attr_type>().map_err(|_| Error::CantConvertToNum)?
        } else {
          return Err(Error::ParseIncomplete);
        }
      )+
      // move to next token for pattern matching
      $var = $request;
      Ok::<$type,Error>(value)
    }
  };
}

pub(in crate::obj_loader) use parse_num;
pub(in crate::obj_loader) use parse_line;
pub(in crate::obj_loader) use skip_to_next_line;

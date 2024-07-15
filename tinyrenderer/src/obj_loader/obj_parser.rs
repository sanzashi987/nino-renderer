use super::defines::{self, ParserError};

macro_rules! parse_num {
  ($var:ident,$type:ty) => {{
    $var
      .parse::<$type>()
      .map_err(|_| ParserError::CantConvertToNum)?
  }};
  ($exp:expr,$type:ty) => {{
    let val = $exp;
    val
      .parse::<$type>()
      .map_err(|_| ParserError::CantConvertToNum)?
  }};
}

macro_rules! parse_token {
    ($iter: expr; $type:ty) => {{
      let result = if let Some(s) = $iter {
        s
          .parse::<$type>()
          .map_err(|_| ParserError::CantConvertToType)
      } else {
        Err(ParserError::ParseIncomplete("not a valiad string".to_string()))
      };
      // move to next token
      result
    }};

    ($iter: expr, $type:ty = $($attr:ident : $attr_type:ty),+) => {
      {
        let mut val = <$type>::zero();
        $(
          if let Some(s) = $iter {
            val.$attr = parse_num!(s, $attr_type);
          } else {
            return Err(ParserError::UnExpectedEndOfLine)
          }
        )+

        Ok::<$type,ParserError>(val)
      }
    };
}





pub fn load_obj(relative_path: &str, mode: defines::ParserMode) -> Result<ObjParser, ParserError> {
  let fullpath = std::path::Path::new(relative_path);
  Ok(ObjParser::new(fullpath, mode)?)
}

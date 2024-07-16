#[derive(Debug)]
pub enum ParserError {
  IoError(std::io::Error),
  NotAValidPath,
  InvalidSyntax(String),
  ParseIncomplete(String),
  UnknownToken(String),
  CantConvertToNum,
  CantConvertToType,
  UnExpectedEndOfLine,
  ModelNotInit,
  MaterialNotFound,
}
#[derive(Debug)]
pub enum ParserMode {
  Lazy,
  None,
}

impl Default for ParserMode {
  fn default() -> Self {
    ParserMode::None
  }
}

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

    ($iter: expr; $type:ty = $($attr:ident : $attr_type:ty),+) => {
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

// only convert parse token's Result to Option
macro_rules! parse_token_ok {
  ($iter: expr; $type:ty) => {
    parse_token!($iter; $type).ok()
  };
  ($iter: expr; $type:ty = $($attr:ident : $attr_type:ty),+) => {
    parse_token!($iter; $type = $($attr: $attr_type),+).ok()
  };
}

pub(super) use parse_num;
pub(super) use parse_token;
pub(super) use parse_token_ok;

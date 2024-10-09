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
  LoaderInstanceLoss,
  TextureError(ImageError),
  MtlNotFound,
}

impl From<io::Error> for ParserError {
  fn from(value: io::Error) -> Self {
    Self::IoError(value)
  }
}

pub type ParserResult = Result<(), ParserError>;

macro_rules! parse_num {
  ($var:ident,$type:ty) => {{
    $var
      .parse::<$type>()
      .map_err(|_| crate::loaders::defines::ParserError::CantConvertToNum)?
  }};
  ($exp:expr,$type:ty) => {{
    let val = $exp;
    val
      .parse::<$type>()
      .map_err(|_| crate::loaders::defines::ParserError::CantConvertToNum)?
  }};
}

macro_rules! parse_token {
    ($iter: expr; $type:ty) => {{
      let result = if let Some(s) = $iter {
        s
          .parse::<$type>()
          .map_err(|_| crate::loaders::defines::ParserError::CantConvertToType)
      } else {
        Err(crate::loaders::defines::ParserError::ParseIncomplete("not a valiad string".to_string()))
      };
      // move to next token
      result
    }};

    ($iter: expr; $type:ty = $($attr:ident : $attr_type:ty),+) => {
      {
        let mut val = <$type>::zero();
        $(
          if let Some(s) = $iter {
            val.$attr = crate::loaders::defines::parse_num!(s, $attr_type);
          } else {
            return Err(crate::loaders::defines::ParserError::UnExpectedEndOfLine)
          }
        )+

        Ok::<$type,crate::loaders::defines::ParserError>(val)
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

use std::io;

use image::ImageError;
pub(super) use parse_num;
pub(super) use parse_token;
pub(super) use parse_token_ok;

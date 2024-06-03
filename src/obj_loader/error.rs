#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  CantConvertToNum,
  UnknownToken(String),
  ExceedComponent,
  EmptyContent,
  ParseIncomplete,
  InvalidSyntax,
  PathNotFound,
}

impl From<std::io::Error> for Error {
  fn from(value: std::io::Error) -> Self {
    Self::IoError(value)
  }
}

pub type ParseResult = Result<(), Error>;

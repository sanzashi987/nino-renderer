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
}

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

pub enum LineParseResult {}

pub fn parse<A, F>(iters: &mut A, f: F) -> Result<(), ParserError>
where
  A: Iterator<Item = String>,
  F: Fn(&str) -> Result<(), ParserError>,
{
  for line in iters {
    let trimmed = line.trim().to_string();
    let mut tokens = trimmed.split_whitespace();

    let token = tokens.next();
    if let Some(s) = token {
      f(s)?;
    }
  }

  Ok(())
}

use super::{error::Error, file_content::FileContent};

pub struct TokenRequester<'a> {
  content: &'a FileContent,
  tokens: std::str::SplitWhitespace<'a>,
  line: u64,
}
#[derive(PartialEq)]
pub enum TokenType<'a> {
  Token(&'a str),
  Nextline,
  Eof,
}

impl<'a> TokenRequester<'a> {
  pub fn new(content: &'a FileContent) -> Result<Self, Error> {
    if content.lines.is_empty() {
      Err(Error::EmptyContent)
    } else {
      Ok(Self {
        content,
        tokens: content.lines[0].split_whitespace(),
        line: 0,
      })
    }
  }

  pub fn request(&mut self) -> TokenType<'a> {
    match self.tokens.next() {
      Some(token) => TokenType::Token(token),
      None => {
        self.line += 1;
        if self.line as usize >= self.content.lines.len() {
          TokenType::Eof
        } else {
          self.tokens = self.content.lines[self.line as usize].split_whitespace();
          TokenType::Nextline
        }
      }
    }
  }
}

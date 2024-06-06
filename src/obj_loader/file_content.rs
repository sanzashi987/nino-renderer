use std::{
  fs::File,
  io::{BufRead, BufReader, Error},
  path::Path,
};

pub struct FileContent {
  pub lines: Vec<String>,
}

impl FileContent {
  pub fn from_file(filename: &Path) -> Result<FileContent, Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let (mut line, mut lines) = (String::new(), vec![] as Vec<String>);

    let mut eof = false;

    while !eof {
      match reader.read_line(&mut line) {
        Ok(len) => {
          if len != 0 {
            lines.push(line.clone());
            line.clear();
          } else {
            eof = true;
          }
        }
        Err(err) => return Err(err),
      }
    }

    Ok(FileContent { lines })
  }
}

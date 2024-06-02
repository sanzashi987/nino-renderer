use std::{
  fs::File,
  io::{BufRead, BufReader, Error},
  path::Path,
};

use crate::math;

struct FileContent {
  lines: Vec<String>,
}

impl FileContent {
  fn from_file(file_name: &Path) -> Result<FileContent, Error> {
    let file = File::open(file_name)?;
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

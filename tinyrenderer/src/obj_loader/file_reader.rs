use std::{
  fs::File,
  io::{BufRead, BufReader, Error},
  path::Path,
};

pub struct FileResult<'a> {
  filename: &'a Path,
  reader: BufReader<File>,
  done: bool,
}

impl<'a> FileResult<'a> {
  pub fn new(filename: &'a Path) -> Result<Self, Error> {
    let file = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    Ok(Self {
      filename,
      reader,
      done: false,
    })
  }

  pub fn reset(&mut self) -> Result<(), Error> {
    let file = File::open(self.filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    self.reader = reader;
    Ok(())
  }
}

impl<'a> Iterator for FileResult<'a> {
  type Item = String;
  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }

    let reader = &mut self.reader;
    let mut res = String::new();

    match reader.read_line(&mut res) {
      Ok(size) => {
        return if size > 0 {
          Some(res)
        } else {
          self.done = true;
          None
          // Err(FileRead::EOF);
        };
      }
      Err(_) => return None,
    }
  }
}

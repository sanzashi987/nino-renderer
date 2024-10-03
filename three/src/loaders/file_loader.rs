use std::{
  fs::File,
  io::{BufRead, BufReader, Error},
};

pub struct FileLoader {
  filename: String,
  reader: BufReader<File>,
  done: bool,
}

impl FileLoader {
  pub fn new(filename: String) -> Result<Self, Error> {
    let file = File::open(&filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    Ok(Self {
      filename,
      reader,
      done: false,
    })
  }

  pub fn reset(&mut self) -> Result<(), Error> {
    let file = File::open(&self.filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    self.reader = reader;
    Ok(())
  }

  pub fn is_done(&self) -> bool {
    self.done
  }
}

impl Iterator for FileLoader {
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

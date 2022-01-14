use std::path::PathBuf;
use std::{fs, io};
use std::io::{ErrorKind, Error};

pub struct Clone {}

impl Clone {
  pub fn execute(path: PathBuf) -> io::Result<()> {
    if !path.exists() {
      return Err(Error::new(ErrorKind::NotFound, "The template could not be found."))
    }

    println!("Cloning {:#?}", path);

    // @see https://doc.rust-lang.org/std/fs/fn.read_dir.html
    let entries: Vec<PathBuf> = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    if entries.len() == 0 {
      return Err(Error::new(ErrorKind::Other, "Your template is empty"))
    }

    for entry in entries {
      println!("{:#?}", entry);
    }

    Ok(())
  }
}

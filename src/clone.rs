use std::path::{PathBuf, Path};
use std::{fs, io};
use std::fs::{DirEntry};
use std::io::{ErrorKind, Error};

pub struct Clone {}

impl Clone {
  // @see https://doc.rust-lang.org/std/fs/fn.read_dir.html
  fn visit_directories(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = Vec::new();

    if dir.is_dir() {
      for entry in fs::read_dir(dir)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
          let mut recursive_entries = Clone::visit_directories(&path)?;
          entries.append(&mut recursive_entries);
        } else {
          // TODO: Create a global list of invalid filenames
          if entry.file_name() != ".DS_Store" {
            entries.push(path);
          }
        }
      }
    }

    Ok(entries)
  }

  pub fn execute(path: PathBuf) -> io::Result<()> {
    if !path.exists() {
      return Err(Error::new(ErrorKind::NotFound, "The template could not be found."))
    }

    println!("Cloning {:#?}", path);

    let entries = Clone::visit_directories(&path)?;

    if entries.len() == 0 {
      return Err(Error::new(ErrorKind::Other, "Your template is empty"))
    }

    for entry in entries {
      println!("{:#?}", entry);
    }

    Ok(())
  }
}

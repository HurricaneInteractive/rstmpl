use dirs;
use std::path::{PathBuf};
use std::fs;

fn get_config_folder_name() -> String {
  String::from(".rstmpl")
}

fn create_templates_folder(home_dir: &PathBuf) -> std::io::Result<()> {
  fs::create_dir_all(home_dir.as_path())?;
  Ok(())
}

pub fn get_global_config_path() -> Result<PathBuf, String> {
  let home_dir = dirs::home_dir();

  match home_dir {
    Some(dir) => {
      let mut path = PathBuf::new();
      path.push(dir);
      path.push(get_config_folder_name());
      path.push("templates");

      if !path.exists() {
        create_templates_folder(&path).unwrap();
      }

      Ok(path)
    }
    _ => {
      Err("Unable to find home directory".to_string())
    }
  }
}

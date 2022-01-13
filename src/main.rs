use clap::{arg, App, AppSettings};
use std::path::{PathBuf};

mod config;

fn main() {
  let matches = App::new("rstmpl")
    .about("Generate your own collection of templates with ease")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(
      App::new("clone")
        .about("Clones a template")
        .arg(arg!(<TEMPLATE> "The template name"))
        .setting(AppSettings::ArgRequiredElseHelp),
    )
    .get_matches();

  match matches.subcommand() {
    Some(("clone", sub_matches)) => {
      let template_name = sub_matches.value_of("TEMPLATE").expect("required");
      let template_dir = config::get_global_config_path().unwrap();
      let mut path = PathBuf::new();
      path.push(template_dir);
      path.push(template_name);

      if path.exists() {
        println!("Cloning {}", template_name);
      } else {
        println!("Unable to find template {}", template_name);
      }
    }
    _ => unreachable!()
  }
}

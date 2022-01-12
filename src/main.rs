use clap::{arg, App, AppSettings};
use dirs;

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
      println!(
        "Cloning {}",
        sub_matches.value_of("TEMPLATE").expect("required")
      );
      println!("Home dir {:#?}", dirs::home_dir().unwrap());
    }
    _ => unreachable!()
  }
}

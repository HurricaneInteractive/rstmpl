use clap::{arg, App, AppSettings};

mod config;
mod clone;
use clone::Clone;

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
      let template_path = config::get_requested_template(sub_matches);
      let result = Clone::execute(template_path);

      match result {
        Err(error) => {
          println!("Error! {}", error);
        }
        _ => {}
      }
    }
    _ => unreachable!()
  }
}

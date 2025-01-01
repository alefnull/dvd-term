use crate::util::{Result, DEFAULT_FONT};
use clap::{arg, value_parser, ArgAction, Command};
use figlet_rs::FIGfont;

mod app;
mod util;

fn main() -> Result<()> {
    let cmd = Command::new("dvd-term")
    .version(env!("CARGO_PKG_VERSION"))
    .about("A bouncing ASCII art DVD logo (or custom text) for the terminal.")
    .help_template("\n
{before-help}{name} {version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}\n")
    .arg(arg!(-t --text <TEXT> "The custom text to use. Defaults to \"DVD\". Can be used multiple times to display multiple \'logos\'")
      .value_parser(value_parser!(String))
      .required(false)
      .action(ArgAction::Append))
    .arg(arg!(-f --font <FONT_PATH> "Specify the path of the figlet font to use")
      .value_parser(value_parser!(String))
      .required(false)
      .action(ArgAction::Set))
    .arg(arg!(-c --color <COLOR> "Initial logo color code (0-255). Defaults to white (15). (https://ss64.com/bash/syntax-colors.html)")
      .value_parser(value_parser!(u8))
      .required(false)
      .action(ArgAction::Set))
    .arg(arg!(-r --random "If included, logo will randomize color when it bounces")
      .value_parser(value_parser!(bool))
      .required(false)
      .action(ArgAction::SetTrue))
    .arg(arg!(-s --speed <SPEED> "The speed of the logo (how many 'cells' to move per second). Defaults to 8")
      .value_parser(value_parser!(u64))
      .required(false)
      .action(ArgAction::Set))
    .arg(arg!(-p --plain "If included, logo will be displayed in plain text instead of converted to ASCII art")
      .value_parser(value_parser!(bool))
      .required(false)
      .action(ArgAction::SetTrue))
    .arg(arg!(-a --art <ART_PATH> "Specify the path of a plain text file with the ASCII art to display")
      .value_parser(value_parser!(String))
      .required(false)
      .action(ArgAction::Set));

    let matches = cmd.clone().get_matches();

    let mut input_text = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|text| text.to_string())
        .collect::<Vec<String>>();
    if input_text.is_empty() {
        input_text.push("DVD".to_string());
    }

    let font_path = if let Some(font_path) = matches.get_one::<String>("font") {
        font_path.to_string()
    } else {
        "".to_string()
    };

    let default_font = FIGfont::from_content(DEFAULT_FONT).unwrap_or(FIGfont::standard().unwrap());

    let font = if font_path.is_empty() {
        default_font
    } else {
        FIGfont::from_file(&font_path).unwrap_or(default_font)
    };

    let color = if let Some(color) = matches.get_one::<u8>("color") {
        *color
    } else {
        15
    };

    let random = if let Some(random) = matches.get_one::<bool>("random") {
        *random
    } else {
        false
    };

    let speed = if let Some(speed) = matches.get_one::<u64>("speed") {
        *speed
    } else {
        8
    };

    let plain = if let Some(plain) = matches.get_one::<bool>("plain") {
        *plain
    } else {
        false
    };

    let art_path = if let Some(art_path) = matches.get_one::<String>("art") {
        art_path.to_string()
    } else {
        "".to_string()
    };

    let mut app = app::App::new(input_text, font, color, random, speed, plain, art_path)?;

    app.run()?;

    Ok(())
}

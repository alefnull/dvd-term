use crate::util::fig_size;
use anyhow::Result;
use clap::{arg, command, value_parser, ArgAction};
use figlet_rs::FIGfont;
use once_cell::sync::Lazy;
use std::fs::read_to_string;

mod app;
mod util;

static DEFAULT_FONT: &str = include_str!("../assets/hash3d.flf");

static MATCHES: Lazy<clap::ArgMatches> = Lazy::new(|| {
  command!()
    .help_template("\n
{before-help}{name} {version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}\n")
    .about("A bouncing ASCII art DVD logo (or custom text) for the terminal.")
    .arg(
      arg!(-t --text <TEXT> "The custom text to use. Defaults to \"DVD\"")
        .value_parser(value_parser!(String))
        .required(false)
        .action(ArgAction::Set),)
    .arg(
      arg!(-f --font <FONT_PATH> "Specify the path of the figlet font to use")
        .value_parser(value_parser!(String))
        .required(false)
        .action(ArgAction::Set),)
    .arg(
      arg!(-c --color <COLOR> "Initial logo color code (0-255). Defaults to white (15). (https://ss64.com/bash/syntax-colors.html)")
        .value_parser(value_parser!(u8))
        .required(false)
        .action(ArgAction::Set),)
    .arg(
      arg!(-r --random "If included, logo will randomize color when it bounces")
        .value_parser(value_parser!(bool))
        .required(false)
        .action(ArgAction::SetTrue),)
    .arg(
      arg!(-s --speed <SPEED> "The speed of the logo (how many 'cells' to move per second). Defaults to 15")
        .value_parser(value_parser!(u32))
        .required(false)
        .action(ArgAction::Set),)
    .arg(
      arg!(-p --plain "If included, logo will be displayed in plain text instead of converted to ASCII art")
        .value_parser(value_parser!(bool))
        .required(false)
        .action(ArgAction::SetTrue),)
    .arg(
      arg!(-a --art <ART_PATH> "Specify the path of a plain text file with the ASCII art to display")
        .value_parser(value_parser!(String))
        .required(false)
        .action(ArgAction::Set),)
    .get_matches()
});

fn main() -> Result<()> {
  let input_text = if let Some(input_text) = MATCHES.get_one::<String>("text") {
    input_text.to_string()
  } else {
    "DVD".to_string()
  };

  let font_path = if let Some(font_path) = MATCHES.get_one::<String>("font") {
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

  let color = if let Some(color) = MATCHES.get_one::<u8>("color") {
    *color
  } else {
    15
  };

  // let random = MATCHES.get_one::<bool>("random").is_some();
  let random = if let Some(random) = MATCHES.get_one::<bool>("random") {
    *random
  } else {
    false
  };

  let speed = if let Some(speed) = MATCHES.get_one::<u32>("speed") {
    *speed as u64
  } else {
    8
  };

  // let plain = MATCHES.get_one::<bool>("plain").is_some();
  let plain = if let Some(plain) = MATCHES.get_one::<bool>("plain") {
    *plain
  } else {
    false
  };

  let art_path = if let Some(art_path) = MATCHES.get_one::<String>("art") {
    art_path.to_string()
  } else {
    "".to_string()
  };

  let art = !art_path.is_empty();

  let mut app = app::App::new(input_text, font, color, random, speed, plain, art)?;

  if art {
    app.logo_string = read_to_string(art_path)?;
    app.logo_size = fig_size(&app.logo_string)?;
  }

  app.run()?;

  Ok(())
}

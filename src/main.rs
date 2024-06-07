use clap::{arg, command, value_parser, ArgAction};
use figlet_rs::FIGfont;
use once_cell::sync::Lazy;

mod app;
mod util;

static DEFAULT_FONT: &str = include_str!("../assets/hash3d.flf");

static MATCHES: Lazy<clap::ArgMatches> = Lazy::new(|| {
    command!()
        .help_template("

{before-help}{name} {version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")
        .about("A bouncing ASCII art DVD logo (or custom text) for the terminal.")
        .arg(
            arg!(-t --text <TEXT> "The custom text to use. Defaults to \"DVD\"")
                .value_parser(value_parser!(String))
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            arg!(-f --font <FONT> "Specify the path of the figlet font to use")
                .value_parser(value_parser!(String))
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            arg!(-c --color <COLOR> "Initial logo color code (0-255). Defaults to white (15). (https://ss64.com/bash/syntax-colors.html)")
                .value_parser(value_parser!(u8))
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            arg!(-r --random "If included, logo will randomize color when it bounces")
                .value_parser(value_parser!(bool))
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-s --speed <SPEED> "The speed of the logo (how many 'cells' to move per second). Defaults to 15")
                .value_parser(value_parser!(u32))
                .required(false)
                .action(ArgAction::Set),
        )
        .get_matches()
});

fn main() {
    let input_text = if let Some(text) = MATCHES.get_one::<String>("text") {
        text.to_string()
    } else {
        "DVD".to_string()
    };

    let font_path = if let Some(font) = MATCHES.get_one::<String>("font") {
        font.to_string()
    } else {
        "".to_string()
    };
    let font = if font_path.is_empty() {
        FIGfont::from_content(DEFAULT_FONT).expect("Failed to load default font")
    } else {
        FIGfont::from_file(&font_path).expect("Failed to load custom font")
    };

    let color = if let Some(color) = MATCHES.get_one::<u8>("color") {
        *color
    } else {
        15
    };

    let random = if let Some(random) = MATCHES.get_one::<bool>("random") {
        *random
    } else {
        false
    };

    let speed = if let Some(speed) = MATCHES.get_one::<u32>("speed") {
        *speed as u64
    } else {
        30
    };

    let mut app = app::App::new(input_text, font, color, random, speed);

    app.run();
}

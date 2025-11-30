use clap::{arg, value_parser, ArgAction, Command};

// default color value (white)
const DEFAULT_COLOR: u8 = 15;
// default speed in cells per second
const DEFAULT_SPEED: u64 = 8;
// default text to display
const DEFAULT_TEXT: &str = "DVD";

pub struct Config {
    // custom text to display (can have multiple logos)
    pub text: Vec<String>,
    // path to custom figlet font file
    pub font_path: String,
    // initial logo color (0-255)
    pub color: u8,
    // whether to randomize color on bounce
    pub random: bool,
    // speed in cells per second
    pub speed: u64,
    // whether to use plain text instead of ASCII art
    pub plain: bool,
    // path to custom ASCII art file
    pub art_path: String,
}

impl Config {
    // parse configuration from command-line arguments
    pub fn from_args() -> Self {
        let cmd = Command::new("dvd-term")
            .version(env!("CARGO_PKG_VERSION"))
            .about("A bouncing ASCII art DVD logo (or custom text) for the terminal.")
            .help_template("\n
{before-help}{name} {version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}\n")
            .arg(arg!(-t --text <TEXT> "The custom text to use. Defaults to \"DVD\". Can be used multiple times to display multiple 'logos'")
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

        let matches = cmd.get_matches();

        // parse text arguments
        let mut text = matches
            .get_many::<String>("text")
            .unwrap_or_default()
            .map(|t| t.to_string())
            .collect::<Vec<String>>();

        if text.is_empty() {
            text.push(DEFAULT_TEXT.to_string());
        }

        Self {
            text,
            font_path: matches
                .get_one::<String>("font")
                .map(|s| s.to_string())
                .unwrap_or_default(),
            color: matches
                .get_one::<u8>("color")
                .copied()
                .unwrap_or(DEFAULT_COLOR),
            random: matches.get_one::<bool>("random").copied().unwrap_or(false),
            speed: matches
                .get_one::<u64>("speed")
                .copied()
                .unwrap_or(DEFAULT_SPEED),
            plain: matches.get_one::<bool>("plain").copied().unwrap_or(false),
            art_path: matches
                .get_one::<String>("art")
                .map(|s| s.to_string())
                .unwrap_or_default(),
        }
    }
}

use clap::{arg, command, value_parser, ArgAction};
use figlet_rs::FIGfont;
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use ruscii::{
    app::{App, Config, State},
    drawing::Pencil,
    keyboard::{Key, KeyEvent},
    spatial::Vec2,
    terminal::{Color, VisualElement, Window},
};
use util::{get_fig_height, get_fig_width, get_random_position, FONT_FILE};

use crate::state::AppState;

mod state;
mod util;

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
    let text = if let Some(text) = MATCHES.get_one::<String>("text") {
        text
    } else {
        "DVD"
    };

    let mut font = FIGfont::from_content(FONT_FILE).expect("Failed to load font");
    if let Some(custom_font) = MATCHES.get_one::<String>("font") {
        font = FIGfont::from_file(custom_font).expect("Failed to load font");
    }

    let color = if let Some(color) = MATCHES.get_one::<u8>("color") {
        Color::Xterm(*color)
    } else {
        Color::White
    };

    let mut randomized = false;

    if let Some(random) = MATCHES.get_one::<bool>("random") {
        randomized = *random;
    };

    let speed = if let Some(speed) = MATCHES.get_one::<u32>("speed") {
        *speed
    } else {
        15
    };

    let mut rng = thread_rng();
    let left: bool = rng.gen();
    let up: bool = rng.gen();

    let dir = Vec2::xy(if left { -1 } else { 1 }, if up { -1 } else { 1 });

    let mut app = App::config(Config::new().fps(speed));

    let mut app_state = AppState::new(
        text.to_string(),
        &font,
        get_random_position(
            app.window().size(),
            Vec2::xy(get_fig_width(text, &font), get_fig_height(text, &font)),
        ),
        dir,
        color,
        randomized,
    );

    let text = font.convert(text).expect("Failed to render text");

    app.run(|state: &mut State, window: &mut Window| {
        window.canvas_mut().set_default_element(&VisualElement {
            background: Color::Xterm(0),
            ..Default::default()
        });
        window.canvas_mut().clear();
        let win_size = window.size();
        for key_event in state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => state.stop(),
                KeyEvent::Pressed(Key::Q) => state.stop(),
                _ => (),
            }
        }

        let mut pencil = Pencil::new(window.canvas_mut());

        app_state.update(win_size);

        for (y, line) in text.to_string().lines().enumerate() {
            pencil.set_foreground(app_state.color);
            pencil.draw_text(line, Vec2::xy(app_state.pos.x, app_state.pos.y + y as i32));
        }
    });
}

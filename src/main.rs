use crate::util::{Result, DEFAULT_FONT};
use figlet_rs::FIGfont;

mod app;
mod config;
mod util;

fn main() -> Result<()> {
    let config = config::Config::from_args();

    // load the figlet font (custom or default)
    let default_font =
        FIGfont::from_content(DEFAULT_FONT).unwrap_or_else(|_| FIGfont::standard().unwrap());

    let font = if config.font_path.is_empty() {
        default_font
    } else {
        FIGfont::from_file(&config.font_path).unwrap_or(default_font)
    };

    // create and run the application
    let mut app = app::App::new(
        config.text,
        font,
        config.color,
        config.random,
        config.speed,
        config.plain,
        config.art_path,
    )?;

    app.run()?;

    Ok(())
}

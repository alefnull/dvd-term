// utility functions and types for the dvd-term application
use figlet_rs::{self, FIGfont};
use rand::{rng, Rng};
use std::io::Error;

// embedded default figlet font
pub static DEFAULT_FONT: &str = include_str!("../../assets/hash3d.flf");

// result type alias for IO operations
pub type Result<T> = core::result::Result<T, Error>;

// 2D vector for position and direction
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    // create a new Vec2 with the given coordinates
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // create a random Vec2 within the given bounds, accounting for offsets
    pub fn rand(width: i32, height: i32, xoff: i32, yoff: i32) -> Self {
        Self {
            x: rng().random_range(0..width - xoff),
            y: rng().random_range(0..height - yoff),
        }
    }

    // create a random direction vector (either 1 or -1 for each axis)
    pub fn rand_dir() -> Self {
        let x = if rng().random() { 1 } else { -1 };
        let y = if rng().random() { 1 } else { -1 };
        Self { x, y }
    }
}

// get the current terminal size as a Vec2
pub fn term_size() -> Result<Vec2> {
    let (cols, rows) = crossterm::terminal::size()?;
    Ok(Vec2::new(cols as i32, rows as i32))
}

// convert input text to ASCII art using the given figlet font
pub(crate) fn figlet(input: &str, font: &FIGfont) -> Result<String> {
    Ok(font
        .convert(input)
        .expect("Failed to convert input to ASCII art.")
        .to_string()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .to_string())
}

// calculate the dimensions of ASCII art text
pub fn fig_size(input: &str) -> Result<Vec2> {
    let w = input.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let h = input.lines().filter(|l| !l.trim().is_empty()).count();

    Ok(Vec2::new(w as i32, h as i32))
}

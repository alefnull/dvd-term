use figlet_rs::{self, FIGfont};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn rand(width: i32, height: i32, xoff: i32, yoff: i32) -> Self {
        Self {
            x: rand::thread_rng().gen_range(0..width - xoff),
            y: rand::thread_rng().gen_range(0..height - yoff),
        }
    }

    pub fn rand_dir() -> Self {
        let x = if rand::thread_rng().gen() { 1 } else { -1 };
        let y = if rand::thread_rng().gen() { 1 } else { -1 };
        Self { x, y }
    }
}

pub fn term_size() -> Vec2 {
    let (cols, rows) = crossterm::terminal::size().expect("Failed to get terminal size"); // (cols, rows)
    Vec2::new(cols as i32, rows as i32)
}

pub(crate) fn figlet(input: &str, font: &FIGfont) -> String {
    font.convert(input)
        .expect("Failed to convert text")
        .to_string()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .to_string()
}

pub fn fig_size(input: &str) -> Vec2 {
    let w = input.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let h = input.lines().filter(|l| !l.trim().is_empty()).count();

    Vec2::new(w as i32, h as i32)
}

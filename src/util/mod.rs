use figlet_rs::FIGfont;
use rand::Rng;
use ruscii::{spatial::Vec2, terminal::Color};

pub const FONT_FILE: &str = include_str!("../../assets/hash3d.flf");

pub fn get_random_position(win_size: Vec2, text_size: Vec2) -> Vec2 {
    let mut rng = rand::thread_rng();
    Vec2::xy(
        rng.gen_range(0..win_size.x - text_size.x),
        rng.gen_range(0..win_size.y - text_size.y),
    )
}

pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();

    Color::Xterm(rng.gen_range(0..255))
}

pub fn get_fig_width(text: &str, font: &FIGfont) -> usize {
    if let Some(fig_text) = font.convert(text) {
        let max_width = fig_text
            .to_string()
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap();
        return max_width;
    }
    0
}

pub fn get_fig_height(text: &str, font: &FIGfont) -> usize {
    if let Some(fig_text) = font.convert(text) {
        let mut max_height = fig_text.height as usize;
        for line in fig_text.to_string().lines() {
            if contains_only_escapes_or_white_spaces(line) {
                max_height -= 1;
            }
        }
        return max_height;
    }
    0
}

pub fn contains_only_escapes_or_white_spaces(text: &str) -> bool {
    text.chars().all(|c| c.is_whitespace() || c == '\x1b')
}

use ruscii::{spatial::Vec2, terminal::Color};

use crate::util::{get_fig_height, get_fig_width, get_random_color};

pub struct AppState {
    pub text: String,
    pub pos: Vec2,
    pub dir: Vec2,
    pub size: Vec2,
    pub randomized: bool,
    pub color: Color,
    pub bottom: usize,
}

impl AppState {
    pub fn new(text: String, pos: Vec2, dir: Vec2, color: Color, randomized: bool) -> Self {
        let size = Vec2::xy(get_fig_width(&text), get_fig_height(&text));
        Self {
            text: text.to_string(),
            pos,
            dir,
            size,
            randomized,
            color,
            bottom: 0,
        }
    }

    pub fn update(&mut self, bounds: Vec2) {
        if self.pos.x + self.size.x >= bounds.x {
            self.dir.x = -1;
            if self.randomized {
                self.color = get_random_color();
            }
        } else if self.pos.x <= 0 {
            self.dir.x = 1;
            if self.randomized {
                self.color = get_random_color();
            }
        }

        if self.pos.y + self.size.y >= bounds.y {
            self.dir.y = -1;
            if self.randomized {
                self.color = get_random_color();
            }
        } else if self.pos.y <= 0 {
            self.dir.y = 1;
            if self.randomized {
                self.color = get_random_color();
            }
        }

        self.pos += self.dir;
    }
}

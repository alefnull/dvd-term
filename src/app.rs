use std::{
    io::{BufWriter, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyEvent},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use figlet_rs::FIGfont;
use rand::Rng;

use crate::util::{fig_size, figlet, term_size, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub color: u8,
    pub char: char,
}

pub struct App {
    pub target: BufWriter<std::io::Stdout>,
    pub input: String,
    pub font: FIGfont,
    pub logo_string: String,
    pub logo_size: Vec2,
    pub canvas_size: Vec2,
    pub color: u8,
    pub speed: u64,
    pub random: bool,
    pub position: Vec2,
    pub direction: Vec2,
    pub running: bool,
    pub plain: bool,
    pub art: bool,
}

impl App {
    pub fn new(
        input_text: String,
        font: FIGfont,
        color: u8,
        random: bool,
        speed: u64,
        plain: bool,
        art: bool,
    ) -> Self {
        let fig_str = figlet(&input_text, &font);
        let fig_size = fig_size(&fig_str);
        Self {
            input: input_text,
            font,
            color,
            random,
            speed,
            target: BufWriter::new(std::io::stdout()),
            logo_string: fig_str,
            canvas_size: term_size(),
            logo_size: fig_size,
            position: Vec2::rand(term_size().x, term_size().y, fig_size.x, fig_size.y),
            direction: Vec2::rand_dir(),
            running: true,
            plain,
            art,
        }
    }

    pub fn update(&mut self) {
        self.canvas_size = term_size();

        // let mut plain = self.fig_size.x >= self.canvas_size.x - (self.fig_size.x / 2)
        //     || self.fig_size.y >= self.canvas_size.y - (self.fig_size.y / 2);
        // if self.plain {
        //     plain = true;
        // }
        let plain = if self.plain {
            true
        } else {
            self.logo_size.x >= self.canvas_size.x - (self.logo_size.x / 2)
                || self.logo_size.y >= self.canvas_size.y - (self.logo_size.y / 2)
        };

        let mut bounce = false;

        if !plain {
            if self.position.x + self.direction.x < 0 {
                bounce = true;
                self.position.x = 0;
                self.direction.x = -self.direction.x;
            } else if self.position.x + self.direction.x + self.logo_size.x >= self.canvas_size.x {
                bounce = true;
                self.position.x = self.canvas_size.x - self.logo_size.x - 1;
                self.direction.x = -self.direction.x;
            }

            if self.position.y + self.direction.y < 0 {
                bounce = true;
                self.position.y = 0;
                self.direction.y = -self.direction.y;
            } else if self.position.y + self.direction.y + self.logo_size.y > self.canvas_size.y {
                bounce = true;
                self.position.y = self.canvas_size.y - self.logo_size.y;
                self.direction.y = -self.direction.y;
            }
        } else {
            if self.position.x + self.direction.x < 0 {
                bounce = true;
                self.position.x = 0;
                self.direction.x = -self.direction.x;
            } else if self.position.x + self.direction.x + self.input.len() as i32
                >= self.canvas_size.x
            {
                bounce = true;
                self.position.x = self.canvas_size.x - self.input.len() as i32 - 1;
                self.direction.x = -self.direction.x;
            }

            if self.position.y + self.direction.y < 0 {
                bounce = true;
                self.position.y = 0;
                self.direction.y = -self.direction.y;
            } else if self.position.y + self.direction.y > self.canvas_size.y - 1 {
                bounce = true;
                self.position.y = self.canvas_size.y - 1;
                self.direction.y = -self.direction.y;
            }
        }

        self.position.x += self.direction.x;
        self.position.y += self.direction.y;

        if self.random && bounce {
            self.color = rand::thread_rng().gen_range(0..=255);
        }
    }

    pub fn draw(&mut self) {
        queue!(self.target, Clear(ClearType::All),).expect("Failed to clear screen");

        let plain = if self.plain {
            true
        } else {
            self.logo_size.x >= self.canvas_size.x - (self.logo_size.x / 2)
                || self.logo_size.y >= self.canvas_size.y - (self.logo_size.y / 2)
        };
        if plain {
            queue!(
                self.target,
                MoveTo(self.position.x as u16, self.position.y as u16),
                SetForegroundColor(Color::AnsiValue(self.color)),
                Print(self.input.as_str())
            )
            .expect("Failed to print text");
        } else {
            for (i, line) in self.logo_string.lines().enumerate() {
                let pos = Vec2::new(self.position.x, self.position.y + i as i32);

                queue!(
                    self.target,
                    MoveTo(pos.x as u16, pos.y as u16),
                    SetForegroundColor(Color::AnsiValue(self.color)),
                    Print(line)
                )
                .expect("Failed to print text");
            }
        }

        self.target.flush().expect("Failed to flush stdout");
    }

    pub fn handle_input(&mut self) {
        let target_frame_time = 1000 / self.speed;

        if poll(Duration::from_millis(target_frame_time)).expect("Failed to poll for events") {
            match read().expect("Failed to read event") {
                Event::Key(KeyEvent {
                    code: crossterm::event::KeyCode::Char('q'),
                    ..
                }) => self.running = false,
                Event::Key(KeyEvent {
                    code: crossterm::event::KeyCode::Esc,
                    ..
                }) => self.running = false,
                Event::Key(KeyEvent {
                    code: crossterm::event::KeyCode::Char('c'),
                    modifiers: crossterm::event::KeyModifiers::CONTROL,
                    state: crossterm::event::KeyEventState::NONE,
                    kind: crossterm::event::KeyEventKind::Press,
                }) => self.running = false,
                _ => (),
            }
        }
    }

    pub fn run(&mut self) {
        enable_raw_mode().expect("Failed to enable raw mode");
        execute!(self.target, EnterAlternateScreen, Hide,)
            .expect("Failed to enter alternate screen");

        while self.running {
            self.update();
            self.draw();
            self.handle_input();
        }

        execute!(self.target, Show, LeaveAlternateScreen)
            .expect("Failed to leave alternate screen");
        disable_raw_mode().expect("Failed to disable raw mode");
    }
}

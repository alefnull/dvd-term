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

pub struct Canvas {
    pub data: Vec<Cell>,
    pub size: Vec2,
    pub def_cell: Cell,
}

impl Canvas {
    pub fn new(size: Vec2, def_cell: &Cell) -> Self {
        let mut data = Vec::new();
        data.resize((size.x * size.y) as usize, *def_cell);
        Self {
            data,
            size,
            def_cell: *def_cell,
        }
    }
}

pub struct App {
    pub canvas: Canvas,
    pub target: BufWriter<std::io::Stdout>,
    pub input: String,
    pub font: FIGfont,
    pub fig_str: String,
    pub color: u8,
    pub speed: u64,
    pub random: bool,
    pub position: Vec2,
    pub direction: Vec2,
    pub running: bool,
}

impl App {
    pub fn new(input_text: String, font: FIGfont, color: u8, random: bool, speed: u64) -> Self {
        let fig_str = figlet(&input_text, &font);
        let fig_size = fig_size(input_text.as_str());
        Self {
            input: input_text,
            font,
            color,
            random,
            speed,
            canvas: Canvas::new(
                term_size(),
                &Cell {
                    color: 0,
                    char: ' ',
                },
            ),
            target: BufWriter::new(std::io::stdout()),
            fig_str,
            position: Vec2::rand(term_size().x, term_size().y, fig_size.x, fig_size.y),
            direction: Vec2::rand_dir(),
            running: true,
        }
    }

    pub fn update(&mut self) {
        self.canvas.size = term_size();

        let mut bounce = false;

        if self.position.x + self.direction.x < 0 {
            bounce = true;
            self.position.x = 0;
            self.direction.x = -self.direction.x;
        } else if self.position.x + self.direction.x + fig_size(self.fig_str.as_str()).x
            >= self.canvas.size.x
        {
            bounce = true;
            self.position.x = self.canvas.size.x - fig_size(self.fig_str.as_str()).x - 1;
            self.direction.x = -self.direction.x;
        }

        if self.position.y + self.direction.y < 0 {
            bounce = true;
            self.position.y = 0;
            self.direction.y = -self.direction.y;
        } else if self.position.y + self.direction.y + fig_size(self.fig_str.as_str()).y
            >= self.canvas.size.y
        {
            bounce = true;
            self.position.y = self.canvas.size.y - fig_size(self.fig_str.as_str()).y - 1;
            self.direction.y = -self.direction.y;
        }

        self.position.x += self.direction.x;
        self.position.y += self.direction.y;

        if bounce {
            self.color = rand::thread_rng().gen_range(0..=255);
        }
    }

    pub fn draw(&mut self) {
        queue!(self.target, Clear(ClearType::All)).unwrap();
        for (i, line) in self.fig_str.lines().enumerate() {
            let pos = Vec2::new(self.position.x, self.position.y + i as i32);
            self.canvas
                .data
                .iter_mut()
                .skip((pos.y * self.canvas.size.x + pos.x) as usize)
                .take(line.chars().count())
                .for_each(|cell| {
                    if cell.char == self.canvas.def_cell.char {
                        return;
                    }
                    cell.char = line.chars().next().unwrap();
                    cell.color = self.color;
                });

            queue!(
                self.target,
                MoveTo(pos.x as u16, pos.y as u16),
                SetForegroundColor(Color::AnsiValue(self.color)),
                Print(line)
            )
            .unwrap();
        }

        self.target.flush().unwrap();
    }

    pub fn handle_input(&mut self) {
        let target_frame_time = 1000 / self.speed;

        if poll(Duration::from_millis(target_frame_time)).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: crossterm::event::KeyCode::Char('q'),
                    ..
                }) => self.running = false,
                Event::Key(KeyEvent {
                    code: crossterm::event::KeyCode::Esc,
                    ..
                }) => self.running = false,
                _ => (),
            }
        }
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        execute!(self.target, EnterAlternateScreen, Hide,).unwrap();

        while self.running {
            self.update();
            self.draw();
            self.handle_input();
        }

        execute!(self.target, Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

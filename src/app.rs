use crate::util::{fig_size, figlet, term_size, Result, Vec2};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use figlet_rs::FIGfont;
use rand::{rng, Rng};
use std::{
    fs::read_to_string,
    io::{BufWriter, Write},
    path::Path,
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub color: u8,
    pub char: char,
}

pub struct App {
    pub target: BufWriter<std::io::Stdout>,
    pub input: Vec<String>,
    pub logo_strings: Vec<String>,
    pub logo_sizes: Vec<Vec2>,
    pub canvas_size: Vec2,
    pub colors: Vec<u8>,
    pub speed: u64,
    pub random: bool,
    pub positions: Vec<Vec2>,
    pub directions: Vec<Vec2>,
    pub running: bool,
    pub plain: bool,
}

impl App {
    pub fn new(
        input_text: Vec<String>,
        font: FIGfont,
        color: u8,
        random: bool,
        speed: u64,
        plain: bool,
        art_path: String,
    ) -> Result<Self> {
        let mut logo_strs: Vec<String> = Vec::new();
        let mut logo_sizes: Vec<Vec2> = Vec::new();

        if !input_text.is_empty() {
            for text in &input_text {
                let logo_str = figlet(text, &font)?;
                logo_sizes.push(fig_size(logo_str.as_str())?);
                logo_strs.push(logo_str);
            }
        }

        let art = !art_path.is_empty();
        if art {
            if !Path::new(&art_path).exists() {
                println!("File not found: {}", art_path,);
                std::process::exit(0);
            } else {
                let art_path = Path::new(&art_path);
                logo_strs.push(read_to_string(art_path)?);
                logo_sizes.push(fig_size(&logo_strs[logo_strs.len() - 1])?);
            }
        }

        let mut colors = Vec::new();
        if random {
            for _ in 0..logo_strs.len() {
                // let mut c = rand::thread_rng().gen_range(1..=231);
                let mut c = rng().random_range(1..=231);
                while c == 16 {
                    // c = rand::thread_rng().gen_range(1..=231);
                    c = rng().random_range(1..=231);
                }
                colors.push(c);
            }
        } else {
            for _ in 0..logo_strs.len() {
                colors.push(color);
            }
        }

        let mut positions: Vec<Vec2> = Vec::new();
        let mut directions: Vec<Vec2> = Vec::new();
        for _ in 0..logo_strs.len() {
            positions.push(Vec2::rand(
                term_size()?.x,
                term_size()?.y,
                logo_sizes[0].x,
                logo_sizes[0].y,
            ));
            directions.push(Vec2::rand_dir());
        }

        Ok(Self {
            input: input_text,
            colors,
            random,
            speed,
            target: BufWriter::new(std::io::stdout()),
            logo_strings: logo_strs,
            logo_sizes,
            canvas_size: term_size()?,
            positions,
            directions,
            running: true,
            plain,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.canvas_size = term_size()?;

        let plain = if self.plain {
            true
        } else {
            let max = self
                .logo_sizes
                .iter()
                .max_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
                .unwrap();
            max.x >= self.canvas_size.x - (max.x / 2) || max.y >= self.canvas_size.y - (max.y / 2)
        };

        let mut bounces: Vec<bool> = vec![false; self.logo_strings.len()];

        if !plain {
            for (i, _) in self.logo_strings.iter().enumerate() {
                if self.positions[i].x + self.directions[i].x < 0 {
                    bounces[i] = true;
                    self.positions[i].x = 0;
                    self.directions[i].x = -self.directions[i].x;
                } else if self.positions[i].x + self.directions[i].x + self.logo_sizes[i].x
                    >= self.canvas_size.x
                {
                    bounces[i] = true;
                    self.positions[i].x = self.canvas_size.x - self.logo_sizes[i].x - 1;
                    self.directions[i].x = -self.directions[i].x;
                }

                if self.positions[i].y + self.directions[i].y < 0 {
                    bounces[i] = true;
                    self.positions[i].y = 0;
                    self.directions[i].y = -self.directions[i].y;
                } else if self.positions[i].y + self.directions[i].y + self.logo_sizes[i].y
                    >= self.canvas_size.y
                {
                    bounces[i] = true;
                    self.positions[i].y = self.canvas_size.y - self.logo_sizes[i].y;
                    self.directions[i].y = -self.directions[i].y;
                }
            }
        } else {
            for (i, _) in self.logo_strings.iter().enumerate() {
                if self.positions[i].x + self.directions[i].x < 0 {
                    bounces[i] = true;
                    self.positions[i].x = 0;
                    self.directions[i].x = -self.directions[i].x;
                } else if self.positions[i].x + self.directions[i].x + self.input[i].len() as i32
                    >= self.canvas_size.x
                {
                    bounces[i] = true;
                    self.positions[i].x = self.canvas_size.x - self.input[i].len() as i32 - 1;
                    self.directions[i].x = -self.directions[i].x;
                }

                if self.positions[i].y + self.directions[i].y < 0 {
                    bounces[i] = true;
                    self.positions[i].y = 0;
                    self.directions[i].y = -self.directions[i].y;
                } else if self.positions[i].y + self.directions[i].y >= self.canvas_size.y {
                    bounces[i] = true;
                    self.positions[i].y = self.canvas_size.y;
                    self.directions[i].y = -self.directions[i].y;
                }
            }
        }

        for (i, _) in self.logo_strings.iter().enumerate() {
            self.positions[i].x += self.directions[i].x;
            self.positions[i].y += self.directions[i].y;

            if self.random && bounces[i] {
                let prev_col = self.colors[i];
                while self.colors[i] == prev_col || self.colors[i] == 16 {
                    // self.colors[i] = rand::thread_rng().gen_range(1..=231);
                    self.colors[i] = rng().random_range(1..=231);
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        queue!(self.target, Clear(ClearType::All),)?;

        let plain = if self.plain {
            true
        } else {
            let max = self.logo_sizes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap();

            max.x >= self.canvas_size.x - (max.x / 2) || max.y >= self.canvas_size.y - (max.y / 2)
        };
        if plain {
            for i in 0..self.logo_strings.len() {
                queue!(
                    self.target,
                    MoveTo(self.positions[i].x as u16, self.positions[i].y as u16),
                    SetForegroundColor(Color::AnsiValue(self.colors[i])),
                    Print(self.input[i].as_str())
                )?;
            }
        } else {
            for i in 0..self.logo_strings.len() {
                for (j, line) in self.logo_strings[i].lines().enumerate() {
                    let pos = Vec2::new(self.positions[i].x, self.positions[i].y + j as i32);

                    for (c, char) in line.chars().enumerate() {
                        if char != ' ' {
                            queue!(
                                self.target,
                                MoveTo(pos.x as u16 + c as u16, pos.y as u16),
                                SetForegroundColor(Color::AnsiValue(self.colors[i])),
                                Print(char)
                            )?;
                        }
                    }
                }
            }
        }

        self.target.flush()?;

        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<()> {
        let target_frame_time = 1000 / self.speed;

        if poll(Duration::from_millis(target_frame_time))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => self.running = false,
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => self.running = false,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    state: KeyEventState::NONE,
                    kind: KeyEventKind::Press,
                }) => self.running = false,
                _ => (),
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(self.target, EnterAlternateScreen, Hide,)?;

        while self.running {
            self.handle_input()?;
            self.update()?;
            self.draw()?;
        }

        execute!(self.target, Show, LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}

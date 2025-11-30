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

// color constants
const MIN_COLOR: u8 = 1;
const MAX_COLOR: u8 = 231;
const BLACK_COLOR: u8 = 16;

// timing constants
const MILLISECONDS_PER_SECOND: u64 = 1000;

// main application state for the bouncing logo screensaver
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
    // create a new App instance with the given configuration
    pub fn new(
        input_text: Vec<String>,
        font: FIGfont,
        color: u8,
        random: bool,
        speed: u64,
        plain: bool,
        art_path: String,
    ) -> Result<Self> {
        let mut logo_strings: Vec<String> = Vec::new();
        let mut logo_sizes: Vec<Vec2> = Vec::new();

        // convert input text to ASCII art using figlet
        if !input_text.is_empty() {
            for text in &input_text {
                let logo_str = figlet(text, &font)?;
                logo_sizes.push(fig_size(logo_str.as_str())?);
                logo_strings.push(logo_str);
            }
        }

        // load custom ASCII art from file if provided
        if !art_path.is_empty() {
            if !Path::new(&art_path).exists() {
                println!("File not found: {art_path}");
                std::process::exit(0);
            }
            let art_content = read_to_string(&art_path)?;
            logo_sizes.push(fig_size(&art_content)?);
            logo_strings.push(art_content);
        }

        // initialize colors for each logo
        let colors = if random {
            (0..logo_strings.len())
                .map(|_| Self::generate_random_color())
                .collect()
        } else {
            vec![color; logo_strings.len()]
        };

        // initialize random positions and directions for each logo
        let canvas_size = term_size()?;
        let mut positions: Vec<Vec2> = Vec::new();
        let mut directions: Vec<Vec2> = Vec::new();

        for size in &logo_sizes {
            positions.push(Vec2::rand(canvas_size.x, canvas_size.y, size.x, size.y));
            directions.push(Vec2::rand_dir());
        }

        Ok(Self {
            input: input_text,
            colors,
            random,
            speed,
            target: BufWriter::new(std::io::stdout()),
            logo_strings,
            logo_sizes,
            canvas_size,
            positions,
            directions,
            running: true,
            plain,
        })
    }

    // generate a random color excluding black (color 16)
    fn generate_random_color() -> u8 {
        let mut color = rng().random_range(MIN_COLOR..=MAX_COLOR);
        while color == BLACK_COLOR {
            color = rng().random_range(MIN_COLOR..=MAX_COLOR);
        }
        color
    }

    // randomize the color for a specific logo
    fn randomize_color(&mut self, index: usize) {
        let prev_color = self.colors[index];
        while self.colors[index] == prev_color || self.colors[index] == BLACK_COLOR {
            self.colors[index] = rng().random_range(MIN_COLOR..=MAX_COLOR);
        }
    }

    // determine if plain text mode should be used based on terminal size
    fn should_use_plain_mode(&self) -> bool {
        if self.plain {
            return true;
        }

        let max_size = self
            .logo_sizes
            .iter()
            .max_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
            .unwrap();

        max_size.x >= self.canvas_size.x - (max_size.x / 2)
            || max_size.y >= self.canvas_size.y - (max_size.y / 2)
    }

    // get the dimensions (width, height) of the logo with the given index
    fn get_logo_dimensions(&self, index: usize, plain: bool) -> (i32, i32) {
        if plain {
            (self.input[index].len() as i32, 1)
        } else {
            (self.logo_sizes[index].x, self.logo_sizes[index].y)
        }
    }

    // check and handle horizontal bounce, returns true if bounced
    fn check_horizontal_bounce(&mut self, index: usize, logo_width: i32) -> bool {
        let next_x = self.positions[index].x + self.directions[index].x;

        if next_x < 0 {
            self.positions[index].x = 0;
            self.directions[index].x = -self.directions[index].x;
            true
        } else if next_x + logo_width >= self.canvas_size.x {
            self.positions[index].x = self.canvas_size.x - logo_width - 1;
            self.directions[index].x = -self.directions[index].x;
            true
        } else {
            false
        }
    }

    // check and handle vertical bounce, returns true if bounced
    fn check_vertical_bounce(&mut self, index: usize, logo_height: i32) -> bool {
        let next_y = self.positions[index].y + self.directions[index].y;

        if next_y < 0 {
            self.positions[index].y = 0;
            self.directions[index].y = -self.directions[index].y;
            true
        } else if next_y + logo_height >= self.canvas_size.y {
            self.positions[index].y = self.canvas_size.y - logo_height;
            self.directions[index].y = -self.directions[index].y;
            true
        } else {
            false
        }
    }

    // update logo positions and handle bouncing
    pub fn update(&mut self) -> Result<()> {
        self.canvas_size = term_size()?;
        let plain = self.should_use_plain_mode();

        // check for bounces and update positions
        for i in 0..self.logo_strings.len() {
            let (logo_width, logo_height) = self.get_logo_dimensions(i, plain);

            let horizontal_bounce = self.check_horizontal_bounce(i, logo_width);
            let vertical_bounce = self.check_vertical_bounce(i, logo_height);
            let bounced = horizontal_bounce || vertical_bounce;

            // update position
            self.positions[i].x += self.directions[i].x;
            self.positions[i].y += self.directions[i].y;

            // randomize color on bounce if enabled
            if self.random && bounced {
                self.randomize_color(i);
            }
        }

        Ok(())
    }

    // render a plain text logo at the given index's position
    fn draw_plain_logo(&mut self, index: usize) -> Result<()> {
        queue!(
            self.target,
            MoveTo(
                self.positions[index].x as u16,
                self.positions[index].y as u16
            ),
            SetForegroundColor(Color::AnsiValue(self.colors[index])),
            Print(self.input[index].as_str())
        )?;
        Ok(())
    }

    // render an ASCII art logo at the given index's position
    fn draw_ascii_logo(&mut self, index: usize) -> Result<()> {
        for (j, line) in self.logo_strings[index].lines().enumerate() {
            let pos = Vec2::new(self.positions[index].x, self.positions[index].y + j as i32);

            for (c, char) in line.chars().enumerate() {
                if char != ' ' {
                    queue!(
                        self.target,
                        MoveTo(pos.x as u16 + c as u16, pos.y as u16),
                        SetForegroundColor(Color::AnsiValue(self.colors[index])),
                        Print(char)
                    )?;
                }
            }
        }
        Ok(())
    }

    // draw all logos to the terminal
    pub fn draw(&mut self) -> Result<()> {
        queue!(self.target, Clear(ClearType::All))?;

        let plain = self.should_use_plain_mode();

        for i in 0..self.logo_strings.len() {
            if plain {
                self.draw_plain_logo(i)?;
            } else {
                self.draw_ascii_logo(i)?;
            }
        }

        self.target.flush()?;
        Ok(())
    }

    // handle keyboard input for quitting the application
    pub fn handle_input(&mut self) -> Result<()> {
        let target_frame_time = MILLISECONDS_PER_SECOND / self.speed;

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

    // main application loop
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(self.target, EnterAlternateScreen, Hide)?;

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

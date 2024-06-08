# dvd-term 

[![CI](https://github.com/alefnull/dvd-term/workflows/CI/badge.svg)](https://github.com/alefnull/dvd-term/actions)
[![CD](https://github.com/alefnull/dvd-term/workflows/CD/badge.svg)](https://github.com/alefnull/dvd-term/actions)
[![Crates.io](https://img.shields.io/crates/v/dvd-term.svg)](https://crates.io/crates/dvd-term)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/dvd-term?label=crates.io%20downloads)](https://crates.io/crates/dvd-term)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/alefnull/dvd-term/total?label=github%20downloads)](https://github.com/alefnull/dvd-term/releases)
[![License](https://img.shields.io/crates/l/dvd-term)](https://img.shields.io/github/actions/workflow/status/alefnull/dvd-term/cd?label=build)

A bouncing ASCII art DVD logo (or custom text) for the terminal. Uses [`figlet-rs`](https://crates.io/crates/figlet-rs) for converting 'logo' text into ASCII art.

![example gif](dvd-term.gif)

## Recent Changes

### [v0.1.31]

* Removed `ruscii` dependency due to some keyboard input issues on non-Windows platforms. rewritten using `crossterm` directly for rendering and corrected input handling.
* Added CLI flag `-p, --plain` to force dvd-term to only display the input string instead of converting to ASCII art.
* Added CLI argument `-a, --art <ART_PATH>` to take a plain text file with a piece of ASCII art to display instead of the default or custom converted input string.

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install dvd-term`

## Usage

```
Usage: dvd-term [OPTIONS]

Options:
      -t, --text <TEXT>    The custom text to use. Defaults to "DVD"
      -f, --font <FONT>    Specify a custom figlet font path
      -c, --color <COLOR>  Initial logo color code (0-255). Defaults to white (15). (https://ss64.com/bash/syntax-colors.html)
      -r, --random         If included, logo will randomize color when it bounces
      -s, --speed <SPEED>  The speed of the logo (how many 'cells' to move per second). Defaults to 15
      -h, --help           Print help
      -V, --version        Print version
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

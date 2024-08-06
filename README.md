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

### [v0.1.4]

* Modified argument parsing to allow multiple `-t, --text <TEXT>` flags to be used, displaying multiple 'logos'.
    * Updated `-h, --help` message to explain new `-t, --text <TEXT>` usage.
* Modified color randomization to disallow 'black' or other too-dark colors.

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install dvd-term`

## Usage

```
Usage: dvd-term [OPTIONS]

Options:
      -t, --text <TEXT>     The custom text to use. Defaults to "DVD". Can be used multiple times to display multiple 'logos'
      -f, --font <FONT>     Specify a custom figlet font path
      -c, --color <COLOR>   Initial logo color code (0-255). Defaults to white (15). (https://ss64.com/bash/syntax-colors.html)
      -r, --random          If included, logo will randomize color when it bounces
      -s, --speed <SPEED>   The speed of the logo (how many 'cells' to move per second). Defaults to 15
      -p, --plain           If included, logo will be displayed in plain text instead of converted to ASCII art
      -a, --art <ART_PATH>  Specify the path of a plain text file with the ASCII art to display
      -h, --help            Print help
      -V, --version         Print version
```

## Contributing (Issues/PRs)

If you encounter any bugs or problems, or you simply have a feature request, please feel free to create an [issue](https://github.com/alefnull/dvd-term/issues) or make a [pull request](https://github.com/alefnull/dvd-term/pulls), and I'll be happy to review and respond, and merge any PRs that pass a quick review.

See [CONTRIBUTING](CONTRIBUTING.md) for details.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

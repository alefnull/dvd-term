# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [v0.1.43]

### Changed

* Updated dependencies:
  * `rand` from `0.8.5` to `0.9.0`
  * `clap` from `4.5.11` to `4.5.27`
* Swapped out deprecated `rand` function calls with their replacements

## [v0.1.4]

### Changed

* Modified argument parsing to allow multiple `-t, --text <TEXT>` flags to be used, displaying multiple 'logos'.
    * Updated `-h, --help` message to explain new `-t, --text <TEXT>` usage.
* Modified color randomization to disallow 'black' or other too-dark colors.

## [v0.1.32]

### Changed

* Implemented some level of error propogation/handling to start moving away from unwrap()/expect().

## [v0.1.31]

### Added

* Added CLI flag `-p, --plain` to force dvd-term to only display the input string instead of converting to ASCII art.
* Added CLI argument `-a, --art <ART_PATH>` to take a plain text file with a piece of ASCII art to display instead of the default or custom converted input string.

### Changed

* Rewrote loop and rendering after removing `ruscii` dependency due to some keyboard input issues on non-Windows platforms. Now using `crossterm` directly for rendering and corrected input handling.

## [v0.1.23]

### Added

* CLI argument to specify custom figlet font path `-f, --font <FONT_PATH>`

## [v0.1.22]

### Fixed

* had some initial build/CI and publishing issues

## [v0.1.0]

### Added

* All the things. Initial commit.

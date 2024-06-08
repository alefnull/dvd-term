# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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

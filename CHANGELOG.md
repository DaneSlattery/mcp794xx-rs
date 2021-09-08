# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.2.1] - 2021-09-08

### Fixed
- Error when converting hours between 24H mode and AM/PM. Thanks to @ppelleti.

## [0.2.0] - 2020-02-09

### Changed
- Changed `get_datetime()` and `set_datetime()` parameter from `DateTime`
  to `chrono::NaiveDateTime`.

### Added
- Methods to set and get date and time using `chrono::NaiveDate` and `chrono::NaiveTime`:
    - `get_time()`
    - `set_time()`
    - `get_date()`
    - `set_date()`
- `chrono` dependency.

### Removed
- `DateTime` data structure was replaced by `chrono::NaiveDateTime`.

## [0.1.0] - 2019-09-15

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future. All changes will be documented in this
CHANGELOG.

[Unreleased]: https://github.com/eldruin/mcp794xx-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/eldruin/mcp794xx-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/mcp794xx-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eldruin/mcp794xx-rs/releases/tag/v0.1.0

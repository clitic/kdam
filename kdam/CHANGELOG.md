# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.2] - 2024-12-11

### Changed

- Bump up pyo3 0.22 -> 0.23

## [0.6.1] - 2024-11-23

### Added

- Support for writing output to tty by @414owen in #23

## [0.6.0] - 2024-11-07

### Changed

- Bump up pyo3 0.21 -> 0.22

## [0.5.2] - 2024-05-13

### Changed

- Bump up formatx 0.2.1 -> 0.2.2
- Bump up pyo3 0.20 -> 0.21
- Bump up rayon 1.8 -> 1.10

## [0.5.1] - 2023-10-05

### Changed

- Bump up pyo3 0.19 -> 0.20
- Bump up windows-sys 0.48.0 -> 0.52.0

## [0.5.0] - 2023-10-05

### Added

- New `notebook` feature.

## [0.4.1] - 2023-08-22

### Fixed

- Panics when using `template` feature with `animation` placeholder.

## [0.4.0] - 2023-07-23

### Added

- `Bar` now implements `Clone` trait.
- `Colour` struct is added for better management of colours when applying them to `Bar`.
- `hide_cursor` and `show_cursor` functions are added to `term` module.
- New cargo features: `derive`, `rayon`, `rich` and `unicode`.
- Support for enabling virtual terminal processing on windows.
- Support for rayon's [ParallelIterator](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html) trait.
- `term::colour` function now supports ANSI 256 colour codes too.

### Changed

- `Animation::CustomWithFill` is merged with `Animation::Custom` enum variant.
- `BarExt` trait methods now return `std::io::Result` type.
- `BarIterator` is renamed to `BarIter` and it's all fields are now private.
- Colorization is now disabled by default.
- `Deref` and `DerefMut` impls are removed from `BarIter`.
- `format_` function name prefix is removed in `format` module.
- Gradient functions are removed from `term::Colorizer` trait.
- Many method names of `Bar` struct are shortened and renamed.
- Most of the fields in `Bar` are made public.
- `RichProgress` and `Column` is now under `rich` feature.
- `RowManager` all fields are private now.
- `term::get_columns_or` is renamed to `term::width` and return `Option` type.
- `term::Writer` struct methods now requires `&[u8]` instead of `&str`.
- Unicode support is disabled by default.

### Fixed

- Apply padding to counter.
- `Bar.leave` is used correctly now.

## [0.3.0] - 2022-12-05

### Added

- `inverse_unit` field in `Bar` for displaying progress rate in inverse form.
- `set_colorize` function in `term` module to enable/disable colours.

### Changed

- Removed `prelude` module.
- Spinner support is now optional and is kept under `spinner` feature.
- `file` field is removed from `Bar` and it is replaced with `BarExt::write_to` method.
- `BarBuilder::build` methods now returns a `Result` type.

## [0.2.7] - 2022-10-11

## [0.2.6] - 2022-10-11

## [0.2.5] - 2022-10-11

### Changed

- Templating abilites are put under an feature named `template`.

### Fixed

- capacity overflow for `RichProgress` for very small window size.
- Gradient progress bar

## [0.2.4] - 2022-08-29

### Fixed

- Fixed `tqdm!(iter)` pattern according to [ISSUE #4](https://github.com/clitic/kdam/issues/4).

## [0.2.3] - 2022-08-19

### Added

- Number of columns of progress bar can now be adjusted by KDAM_NCOLS envoirnment variable.
- New `RowManager` for managing many progress bars. [ISSUE #3](https://github.com/clitic/kdam/issues/3)
- New `Bar.bar_format` field.
- New `Animation::CustomWithFill` variant.
- Better terminal colorization through `rgb(255,255,255)` and `gradient(#5A56E0,#EE6FF8)` values support.
- Now supports spinners.
    - `Spinner`
    - `tqdm!(spinner=..)`

### Changed

-  `eprint_at!` replaced with `Writer::Stderr.print_at`.
- `BarMethods` renamed to `BarExt`.
- `wrap` field removed from `Bar`.

### Fixed

- Fixed `BarMethods::clear` method for bar position above 0.
  
## [0.2.2] - 2022-07-24

### Added

- New `BarBuilder` struct.
- Implemented `From<&str>` for `Animation` and `term::Writer`.

### Changed

- All fields in `Bar` are private.
- `tqdm` macro now uses `BarBuilder` for setting values instead of setting values in `Bar` directly.
- `&str` consuming methods are switched with generic `Into<String>` trait.
- `term::get_columns` is now renamed to `term::get_columns_or`.

## [0.2.1] - 2022-07-21

### Added

- Added `reset` method `BarMethods` trait.
- New `initial` field for `Bar`.

## [0.2.0] - 2022-07-21

### Added

- Optimizations for printing mechanism.
- Optimized rich pulsating bars.

### Changed

- There are many changes in API please read [documentation](https://docs.rs/kdam/0.2.0/kdam/index.html).

### Fixed

- Fixed `Animation::Classic` animation lags.

## [0.1.7] - 2022-05-18

### Fixed

- Fixed displaying counter instead of total value.

## [0.1.6] - 2022-05-11

### Added

- Several optimizations (reduced memory consumption by 30%).
- [rich.progress](https://rich.readthedocs.io/en/latest/progress.html) style progress bar support with custom builder.
- Monitor mode is now supported. Use `monitor`.
- New `wrap` field for `Bar`.

### Changed

- `lock` module has now `AtomicBool` lock.
- `ascii` and few more fields are removed from `Bar`.
- `BarInternal` is now merged into `Bar`.
- All `u64` data types are changed to `usize`.
- Spinner removed from unbounded progress bar

### Fixed

- Unexcepted default print with fira code animation.
- `refresh` method fixed when using `max_fps = true`.
- Fixed `set_ncols` method to use full terminal length.

## [0.1.2] - 2022-04-28

### Added

- [Fira Code](https://github.com/tonsky/FiraCode) animation style i.e. `Animation::FiraCode`.
- Spinner for unknown length progress bar.
- A method to `set_position` is added to `Bar` by referring issue [#1](https://github.com/clitic/kdam/issues/1)
- Support to take input into a `String` by method `input`.

### Fixed

- `set_colour` method of `Bar` is fixed.

## [0.1.0] - 2022-03-20

[Unreleased]: https://github.com/clitic/kdam/compare/0.6.2...HEAD
[0.6.2]: https://github.com/clitic/kdam/compare/0.6.1...0.6.2
[0.6.1]: https://github.com/clitic/kdam/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/clitic/kdam/compare/0.5.2...0.6.0
[0.5.2]: https://github.com/clitic/kdam/compare/v0.5.1...0.5.2
[0.5.1]: https://github.com/clitic/kdam/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/clitic/kdam/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/clitic/kdam/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/clitic/kdam/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/clitic/kdam/compare/799b34c...v0.3.0
[0.2.7]: https://github.com/clitic/kdam/compare/a206ef9...799b34c
[0.2.6]: https://github.com/clitic/kdam/compare/7b6497d...a206ef9
[0.2.5]: https://github.com/clitic/kdam/compare/fd14805...7b6497d
[0.2.4]: https://github.com/clitic/kdam/compare/970d9d9...fd14805
[0.2.3]: https://github.com/clitic/kdam/compare/15a5398...970d9d9
[0.2.2]: https://github.com/clitic/kdam/compare/8dee1ec...15a5398
[0.2.1]: https://github.com/clitic/kdam/compare/80e2ea0...8dee1ec
[0.2.0]: https://github.com/clitic/kdam/compare/2e500d0...80e2ea0
[0.1.7]: https://github.com/clitic/kdam/compare/212923c...2e500d0
[0.1.6]: https://github.com/clitic/kdam/compare/323c3fa...212923c
[0.1.2]: https://github.com/clitic/kdam/compare/3f910c3...323c3fa
[0.1.0]: https://github.com/clitic/kdam/compare/58b20a4...3f910c3

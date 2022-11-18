# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Added

- `inverse_unit` field in `kdam::Bar` for displaying progress rate in inverse form.

## Changed

- Removed `prelude` module.
- Spinner support is now optional and is kept under `spinner` feature.
- `file` field is removed from `Bar` and it is replaced with `writer` feature.
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
- New `kdam::RowManager` for managing many progress bars. [ISSUE #3](https://github.com/clitic/kdam/issues/3)
- New `kdam::Bar.bar_format` field.
- New `kdam::Animation::CustomWithFill` variant.
- Better terminal colorization through `rgb(255,255,255)` and `gradient(#5A56E0,#EE6FF8)` values support.
- Now supports spinners.
    - `kdam::Spinner`
    - `tqdm!(spinner=..)`

### Changed

-  `eprint_at!` replaced with `Writer::Stderr.print_at`.
- `kdam::BarMethods` renamed to `kdam::BarExt`.
- `wrap` field removed from `kdam::Bar`.

### Fixed

- Fixed `BarMethods::clear` method for bar position above 0.
  
## [0.2.2] - 2022-07-24

### Added

- New `kdam::BarBuilder` struct.
- Implemented `From<&str>` for `kdam::Animation` and `kdam::term::Writer`.

### Changed

- All fields in `kdam::Bar` are private.
- `tqdm` macro now uses `kdam::BarBuilder` for setting values instead of setting values in `kdam::Bar` directly.
- `&str` consuming methods are switched with generic `Into<String>` trait.
- `kdam::term::get_columns` is now renamed to `kdam::term::get_columns_or`.

## [0.2.1] - 2022-07-21

### Added

- Added `reset` method `BarMethods` trait.
- New `initial` field for `kdam::Bar`.

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
- Monitor mode is now supported. Use `kdam::monitor`.
- New `wrap` field for `kdam::Bar`.

### Changed

- `lock` module has now `AtomicBool` lock.
- `ascii` and few more fields are removed from `kdam::Bar`.
- `BarInternal` is now merged into `kdam::Bar`.
- All `u64` data types are changed to `usize`.
- Spinner removed from unbounded progress bar

### Fixed

- Unexcepted default print with fira code animation.
- `refresh` method fixed when using `max_fps = true`.
- Fixed `set_ncols` method to use full terminal length.

## [0.1.2] - 2022-04-28

### Added

- [Fira Code](https://github.com/tonsky/FiraCode) animation style i.e. `kdam::Animation::FiraCode`.
- Spinner for unknown length progress bar.
- A method to `set_position` is added to `kdam::Bar` by referring issue [#1](https://github.com/clitic/kdam/issues/1)
- Support to take input into a `String` by method `input`.

### Fixed

- `set_colour` method of `kdam::Bar` is fixed.

## [0.1.0] - 2022-03-20

[Unreleased]: https://github.com/clitic/kdam/compare/799b34c...HEAD
[0.3.0]: https://github.com/clitic/kdam/compare/799b34c...
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

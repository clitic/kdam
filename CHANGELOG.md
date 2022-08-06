# kdam Changelog (DD/MM/YYYY)

## 0.2.3 (dev)

Features:

- Number of columns of progress bar can now be adjusted by KDAM_NCOLS envoirnment variable.
- New `kdam::RowManager` for managing many progress bars. [ISSUE #3](https://github.com/clitic/kdam/issues/3)
- New `kdam::Bar.bar_format` field.
- New `kdam::Animation::CustomWithFill` variant.

Changes:

-  `eprint_at!` replaced with `Writer::Stderr.print_at`.

Bug Fixes:

- Fixed `BarMethods.clear` method for bar position above 0.
  
## 0.2.2 (24/07/2022)

Features:

- New `kdam::BarBuilder` struct.
- Implemented `From<&str>` for `kdam::Animation` and `kdam::term::Writer`.

Changes:

- All fields in `kdam::Bar` are private.
- `tqdm` macro now uses `kdam::BarBuilder` for setting values instead of setting values in `kdam::Bar` directly.
- `&str` consuming methods are switched with generic `Into<String>` trait.
- `kdam::term::get_columns` is now renamed to `kdam::term::get_columns_or`.

## 0.2.1 (21/07/2022)

Features:

- Added `reset` method `BarMethods` trait.
- New `initial` field for `kdam::Bar`.

## 0.2.0 (21/07/2022)

Features:

- Optimizations for printing mechanism.
- Optimized rich pulsating bars.

Changes:

- There are many changes in API please read [documentation](https://docs.rs/kdam/0.2.0/kdam/index.html).

Bug Fixes:

- Fixed `Animation::Classic` animation lags.

## 0.1.7 (18/05/2022)

Bug Fixes:

- Fixed displaying counter instead of total value.

## 0.1.6 (11/05/2022)

Features:

- Several optimizations (reduced memory consumption by 30%).
- [rich.progress](https://rich.readthedocs.io/en/latest/progress.html) style progress bar support with custom builder.
- Monitor mode is now supported. Use `kdam::monitor`.
- New `wrap` field for `kdam::Bar`.

Changes:

- `lock` module has now `AtomicBool` lock.
- `ascii` and few more fields are removed from `kdam::Bar`.
- `BarInternal` is now merged into `kdam::Bar`.
- All `u64` data types are changed to `usize`.
- Spinner removed from unbounded progress bar

Bug Fixes:

- Unexcepted default print with fira code animation.
- `refresh` method fixed when using `max_fps = true`.
- Fixed `set_ncols` method to use full terminal length.

## 0.1.2 (28/04/2022)

Features:

- [Fira Code](https://github.com/tonsky/FiraCode) animation style i.e. `kdam::Animation::FiraCode`.
- Spinner for unknown length progress bar.
- A method to `set_position` is added to `kdam::Bar` by referring issue [#1](https://github.com/clitic/kdam/issues/1)
- Support to take input into a `String` by method `input`.

Bug Fixes:

- `set_colour` method of `kdam::Bar` is fixed.

## 0.1.0 (20/03/2022)

Features:

- Initial release

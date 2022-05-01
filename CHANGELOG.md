# kdam Changelog (DD/MM/YYYY)

## 0.1.3 (29/04/2022)

Features:

- New `kdam::finish` function to print newline.
- New `wrap` field for `kdam::Bar`.

Changes:

- `lock` module has now `AtomicBool` lock.

Bug Fixes:

- Unexcepted default print with fira code animation.
- `refresh` method fixed when using `max_fps = true`.

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

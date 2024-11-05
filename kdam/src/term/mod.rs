//! Terminal related utilities.

use std::io::{stderr, Result, Write};

mod colours;
mod writer;

pub use colours::{colour, init, Colorizer};
pub use writer::{InitializedOutput, Writer};

/// Hide cursor.
pub fn hide_cursor() -> Result<()> {
    stderr().write_all(b"\x1b[?25l")
}

/// Show cursor.
pub fn show_cursor() -> Result<()> {
    stderr().write_all(b"\x1b[?25h")
}

/// Get terminal width.
pub fn width() -> Option<u16> {
    terminal_size::terminal_size().map(|(w, _)| w.0)
}

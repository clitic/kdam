//! Terminal related utilities.

use std::io::Result;

mod colours;
mod writer;

pub use colours::{colour, init, Colorizer};
pub use writer::Writer;

/// Hide cursor.
pub fn hide_cursor() -> Result<()> {
    Writer::Stderr.print("\x1b[?25l".as_bytes())
}

/// Show cursor.
pub fn show_cursor() -> Result<()> {
    Writer::Stderr.print("\x1b[?25h".as_bytes())
}

/// Get terminal width.
pub fn width() -> Option<u16> {
    terminal_size::terminal_size().map(|(w, _)| w.0)
}

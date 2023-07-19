//! Terminal related utilities.

mod colours;
mod writer;

pub use colours::*;
pub use writer::*;

/// Get terminal width.
pub fn width() -> Option<u16> {
    terminal_size::terminal_size().map(|(w, _)| w.0)
}

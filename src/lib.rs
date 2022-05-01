// #![doc=include_str!("../README.md")]
//! A console progress bar library for Rust.

mod iterator_bar;
mod std_bar;
mod styles;
mod term;
mod tqdm_macro;

pub mod format;
pub mod lock;

pub use iterator_bar::{BarIterator, BarProgress};
pub use std_bar::Bar;
pub use styles::{Animation, Output};

/// Prints new line charcter n times to the given output location.
pub fn finish(n: usize, location: Output) {
    match location {
        Output::Stderr => term::write_to_stderr(format_args!("{}", "\n".repeat(n))),
        Output::Stdout => term::write_to_stderr(format_args!("{}", "\n".repeat(n))),
    }
}

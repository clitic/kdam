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

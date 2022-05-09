// #![doc=include_str!("../README.md")]
//! A console progress bar library for Rust.

mod iterator_bar;
mod monitor;
mod rich;
mod std_bar;
mod styles;
mod tqdm_macro;
mod utils;

pub mod format;
pub mod lock;
pub mod term;

pub use iterator_bar::{BarIterator, BarProgress};
pub use monitor::monitor;
pub use rich::{Column, RichProgress};
pub use std_bar::Bar;
pub use styles::Animation;
pub use utils::finish;

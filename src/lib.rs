// #![doc=include_str!("../README.md")]

mod internal;
mod iterator_bar;
mod multi_bar;
mod std_bar;
mod styles;
mod tqdm_macro;

pub mod format;
pub mod term;
pub use iterator_bar::{BarIterator, BarProgress};
pub use multi_bar::BarMulti;
pub use std_bar::Bar;
pub use styles::Animation;

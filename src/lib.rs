#![doc=include_str!("../README.md")]

mod internal;
mod iterator_bar;
mod multi_bar;
mod std_bar;
mod styles;
mod tqdm_macro;

pub mod format;
pub mod term;
pub use iterator_bar::{BarIter, BarIterStruct};
pub use multi_bar::MultiBar;
pub use std_bar::Bar;
pub use styles::Animation;

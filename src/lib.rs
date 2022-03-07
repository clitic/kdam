// #![doc=include_str!("../README.md")]
//! # Examples And Advance Usage
//!
//! - [Description and additional stats](https://github.com/clitic/kdam/blob/main/examples/desc_stats.rs)
//! - [Nested progress bar](https://github.com/clitic/kdam/blob/main/examples/nested.rs)
//! - [Multiple progress bar](https://github.com/clitic/kdam/blob/main/examples/multiple.rs)
//! - [Download a file](https://github.com/clitic/kdam/blob/main/examples/file_download/src/main.rs)
//! - [Copy a file](https://github.com/clitic/kdam/blob/main/examples/file_copy.rs)

mod internal;
mod iterator_bar;
mod std_bar;
mod styles;
mod tqdm_macro;

pub mod format;
pub mod term;
pub use iterator_bar::{BarIterator, BarProgress};
pub use std_bar::Bar;
pub use styles::Animation;

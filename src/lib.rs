// #![doc=include_str!("../README.md")]
//! Ultimate Console Progress Bar For Rust.
//!
//! Inspired by [tqdm](https://github.com/tqdm/tqdm) & [rich.progress](https://rich.readthedocs.io/en/latest/progress.html)
//! 
//! ## Quick Insight
//! 
//! - [kdam::Bar](https://docs.rs/kdam/latest/kdam/struct.Bar.html) can be used to create progress bars like tqdm.
//! - [kdam::RichProgress](https://docs.rs/kdam/latest/kdam/struct.RichProgress.html) can be used to create progress bars like rich.progress.
//! 
//! ## Examples
//! 
//! - [One Page Usage](https://github.com/clitic/kdam#usage)
//! - [Project Examples](https://github.com/clitic/kdam/tree/main/examples)

mod iterator_bar;
mod monitor;
mod rich;
mod std_bar;
mod styles;
mod tqdm_macro;

pub mod format;
pub mod lock;
pub mod term;

pub use iterator_bar::{BarIterator, BarProgress};
pub use monitor::{monitor, monitor_rich};
pub use rich::{Column, RichProgress};
pub use std_bar::Bar;
pub use styles::Animation;

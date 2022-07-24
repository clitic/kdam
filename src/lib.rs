// #![doc=include_str!("../README.md")]
//! Ultimate console progress bar for Rust.
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
//! 
//! ```rust
//! use kdam::prelude::*;
//! 
//! fn main() {
//!     let mut pb = tqdm!(total = 100);
//! 
//!     for _ in 0..100 {
//!         pb.update(1);
//!     }
//! 
//!     eprint!("\n");
//! }
//! ```

mod bar;
mod iterator;
mod rich;
mod styles;
mod macros;

pub mod format;
pub mod lock;
pub mod monitor;
pub mod prelude;
pub mod term;

pub use bar::{Bar, BarBuilder};
pub use iterator::BarIterator;
pub use rich::{Column, RichProgress};
pub use styles::Animation;

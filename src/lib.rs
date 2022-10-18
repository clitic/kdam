#![cfg_attr(docsrs, feature(doc_cfg))]
// #![doc=include_str!("../README.md")]

//! Ultimate console progress bar for Rust.
//!
//! Inspired by [tqdm](https://github.com/tqdm/tqdm) & [rich.progress](https://rich.readthedocs.io/en/latest/progress.html)
//!
//! ## Quick Insight
//!
//! - [Bar](crate::Bar) can be used to create progress bars like tqdm.
//! - [RichProgress](crate::RichProgress) can be used to create progress bars like rich.progress.
//!
//! ## Examples
//!
//! - [One Page Usage](https://github.com/clitic/kdam#usage)
//! - [Project Examples](https://github.com/clitic/kdam/tree/main/examples)
//!
//! ```
//! use kdam::{tqdm, BarExt};
//!
//! let mut pb = tqdm!(total = 100);
//!
//! for _ in 0..100 {
//!     pb.update(1);
//! }
//!
//! eprint!("\n");
//! ```
//!
//! ## Cargo Features
//! 
//! - **gradient**: Enables gradient colours for progress bars and printing text.
//! - **spinner**: Enables support for using spinners. 
//! - **template**: Enables templating capabilities for [Bar](crate::Bar).

mod progress;
mod styles;
mod thread;

pub mod term;

pub use styles::format;
pub use thread::monitor;

pub use progress::{Bar, BarBuilder, BarExt, BarIterator, Column, RichProgress, TqdmIterator};
pub use styles::Animation;
pub use thread::RowManager;

#[cfg(feature = "spinner")]
pub use styles::Spinner;

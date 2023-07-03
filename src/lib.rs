#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc=include_str!("../DOCS.md")]

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

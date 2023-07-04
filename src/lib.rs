#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc=include_str!("../DOCS.md")]

mod progress;
mod styles;
mod thread;

pub mod term;

pub use styles::format;
pub use thread::monitor;

pub use progress::{Bar, BarBuilder, BarExt, BarIterator, TqdmIterator};
pub use styles::Animation;
pub use thread::RowManager;

#[cfg(feature = "rich")]
mod rich;

#[cfg(feature = "rich")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich")))]
pub use rich::{Column, RichProgress};

#[cfg(feature = "spinner")]
mod spinner;

#[cfg(feature = "spinner")]
#[cfg_attr(docsrs, doc(cfg(feature = "spinner")))]
pub use spinner::Spinner;

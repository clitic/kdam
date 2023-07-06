#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc=include_str!("../DOCS.md")]

mod thread;
mod std;
mod iterator;
mod utils;

pub mod format;
pub mod term;

pub use thread::monitor;

pub use iterator::{BarIterator, TqdmIterator};
pub use thread::RowManager;
pub use crate::std::{Bar, BarBuilder, BarExt, Animation, Colour};

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

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc=include_str!("../DOCS.md")]

mod std;
mod utils;

pub mod format;
pub mod lock;
pub mod term;

pub use crate::std::{
    monitor, Animation, Bar, BarBuilder, BarExt, BarIterator, Colour, RowManager, TqdmIterator,
};

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

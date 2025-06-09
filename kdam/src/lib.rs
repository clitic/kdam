#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]
#![doc=include_str!("../DOCS.md")]

mod std;
mod utils;

pub mod format;
pub mod term;

pub use crate::std::{
    monitor, Animation, Bar, BarBuilder, BarExt, BarIter, Colour, RowManager, TqdmIterator,
};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use kdam_derive::BarExt;

#[cfg(feature = "notebook")]
#[cfg_attr(docsrs, doc(cfg(feature = "notebook")))]
pub use crate::std::set_notebook;

#[cfg(feature = "rayon")]
#[cfg_attr(docsrs, doc(cfg(feature = "rayon")))]
pub use rayon;

#[cfg(feature = "rayon")]
#[cfg_attr(docsrs, doc(cfg(feature = "rayon")))]
pub use crate::std::TqdmParallelIterator;

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

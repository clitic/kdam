mod bar;
mod extension;
mod iterator;
mod manager;
mod styles;

pub mod monitor;

pub use bar::{Bar, BarBuilder};
pub use extension::BarExt;
pub use iterator::{BarIter, TqdmIterator};
pub use manager::RowManager;
pub use styles::{Animation, Colour};

#[cfg(feature = "notebook")]
mod notebook;

#[cfg(feature = "notebook")]
pub use notebook::set_notebook;

#[cfg(feature = "rayon")]
pub use iterator::TqdmParallelIterator;

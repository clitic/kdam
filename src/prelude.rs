//! Import traits and macros in current scope.
//!
//! ```rust
//! use kdam::prelude::*;
//! ```

// Traits
pub use crate::progress::{BarExt, TqdmIterator};
pub use crate::term::Colorizer;

// Macros
pub use crate::tqdm;

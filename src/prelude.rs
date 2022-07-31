//! Import traits and macros in current scope.
//! 
//! ```rust
//! use kdam::prelude::*;
//! ```

// Traits
pub use crate::iterator::TqdmIterator;
pub use crate::bar::BarMethods;
pub use crate::term::Colorizer;

// Macros
pub use crate::tqdm;

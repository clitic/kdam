mod animation;

#[cfg(feature = "spinner")]
mod spinner;

pub mod format;
pub mod rich;

pub use animation::*;

#[cfg(feature = "spinner")]
#[cfg_attr(docsrs, doc(cfg(feature = "spinner")))]
pub use spinner::Spinner;

pub mod fmt_data;
mod internal;
mod iterator_bar;
mod multi_bar;
mod std_bar;
mod styles;
pub mod term;

pub use multi_bar::MultiBar;
pub use std_bar::Bar;
pub use styles::Animation;
pub use iterator_bar::BarIter;
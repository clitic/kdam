// #![doc=include_str!("../README.md")]
//! A console progress bar library for Rust.

mod iterator_bar;
mod std_bar;
mod styles;
mod term;
mod tqdm_macro;

pub mod format;
pub mod lock;

pub use iterator_bar::{BarIterator, BarProgress};
pub use std_bar::Bar;
pub use styles::{Animation, Output};

/// Prints new line charcter n times to the given output location.
pub fn finish(n: usize, location: Output) {
    match location {
        Output::Stderr => term::write_to_stderr(format_args!("{}", "\n".repeat(n))),
        Output::Stdout => term::write_to_stderr(format_args!("{}", "\n".repeat(n))),
    }
}

use std::sync::{Arc, Mutex};
use std::thread;

pub fn monitor(pb: Bar, maxinterval: f32) -> (Arc<Mutex<Bar>>, thread::JoinHandle<()>) {
    let pb_arc = Arc::new(Mutex::new(pb));
    let pb_arc_clone = pb_arc.clone();

    let handle = thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_secs_f32(maxinterval));
        let mut pb_monitor = pb_arc_clone.lock().unwrap();

        if pb_monitor.n >= pb_monitor.total {
            break;
        }

        pb_monitor.refresh();
    });

    (pb_arc, handle)
}

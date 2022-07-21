//! Monitor mode for progress bars.

use std::sync::{Arc, Mutex};
use std::thread;

use crate::prelude::*;
use crate::{Bar, RichProgress};

/// Monitor mode for `kdam::Bar`
///
/// # Example
///
/// ```no_run
/// use kdam::prelude::*;
///
/// fn main() {
///     let pb = tqdm!(total = 100, force_refresh = true);
///     let (pb_arc, monitor_thread) = kdam::monitor::bar(pb, 1.0);
///
///     for _ in 0..100 {
///         pb_arc.lock().unwrap().update(1);
///         std::thread::sleep(std::time::Duration::from_secs_f32(3.0));
///     }
///
///     monitor_thread.join().unwrap();
///     eprint!("\n");
/// }
/// ```
pub fn bar(pb: Bar, maxinterval: f32) -> (Arc<Mutex<Bar>>, thread::JoinHandle<()>) {
    let pb_arc = Arc::new(Mutex::new(pb));
    let pb_arc_clone = pb_arc.clone();

    let handle = thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_secs_f32(maxinterval));
        let mut pb_monitor = pb_arc_clone.lock().unwrap();

        if pb_monitor.counter() >= pb_monitor.total {
            break;
        }

        pb_monitor.refresh();
    });

    (pb_arc, handle)
}

/// Monitor mode for `kdam::RichProgress`. See `kdam::monitor::bar` for example usecase.
pub fn rich(
    pb: RichProgress,
    maxinterval: f32,
) -> (Arc<Mutex<RichProgress>>, thread::JoinHandle<()>) {
    let pb_arc = Arc::new(Mutex::new(pb));
    let pb_arc_clone = pb_arc.clone();

    let handle = thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_secs_f32(maxinterval));
        let mut pb_monitor = pb_arc_clone.lock().unwrap();

        if pb_monitor.pb.counter() >= pb_monitor.pb.total {
            break;
        }

        pb_monitor.refresh();
    });

    (pb_arc, handle)
}

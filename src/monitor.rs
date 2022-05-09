use std::sync::{Arc, Mutex};
use std::thread;

use crate::std_bar::Bar;

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

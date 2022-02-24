use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use kdam::tqdm;

fn main() {
    let pb_arc = Arc::new(Mutex::new(tqdm!(total = 10)));
    pb_arc.lock().unwrap().refresh();
    let mut handles = vec![];

    for _ in 0..10 {
        let pb_arc = Arc::clone(&pb_arc);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs_f32(1.0));
            let mut pb = pb_arc.lock().unwrap();
            pb.update(1);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

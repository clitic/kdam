use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use kdam::Bar;

fn main() {
    let pb = Arc::new(Mutex::new(Bar::new(10)));
    let mut handles = vec![];
    pb.lock().unwrap().refresh();

    for _ in 0..10 {
        let pb = Arc::clone(&pb);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs_f32(1.0));
            let mut pb_ref = pb.lock().unwrap();
            pb_ref.update(1);
            pb_ref.refresh();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

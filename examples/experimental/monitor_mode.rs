use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use kdam::tqdm;

fn main() {
    let pb = Arc::new(RwLock::new(tqdm!(total = 100)));

    let pb_c1 = Arc::clone(&pb);
    thread::spawn(move || {
        println!("starting monitor with 1s interval");
        pb_c1.write().unwrap().monitor(1.0);
    });
    let pb_c2 = Arc::clone(&pb);
    for _ in 0..100 {
        println!("locking pb");
        pb_c2.write().unwrap().update(1);
        println!("sleeping for 3s");
        thread::sleep(Duration::from_secs_f32(3.0));
    }
}

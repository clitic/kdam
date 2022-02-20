use std::thread;
use std::time::Duration;
// use std::sync::{Arc, Mutex};
use kdam::Bar;

fn main() {
    let mut pb = Bar::new(100);



    println!("pop1");
    pb.monitor(1.0);

    println!("pop2");
    for _ in 0..100 {
        thread::sleep(Duration::from_secs_f32(3.0));
        pb.update(1);
    }

    // handle.join().unwrap();
}

// use std::thread;
// use std::time::Duration;
// use std::sync::Arc;
// use std::sync::{Arc, RwLock};
// use kdam::Bar;
use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total=10000000);

    for _ in 0..10000000 {
        pb.update(1);
    }
    // pb.update(1);

    // println!("{:?}", "ds");

    // let pb = Arc::new(RwLock::new(Bar::new(100)));

    // let pb1 = Arc::clone(&pb);

    // thread::spawn(move || {
    //     pb1.write().unwrap().monitor(1.0);
    // });
    // for _ in 0..100 {
    //     thread::sleep(Duration::from_secs_f32(3.0));
    //     // println!("pop1");
    //     pb.write().unwrap().update(1);
    //     // println!("pop2");
    // }

    // handle.join().unwrap();
}

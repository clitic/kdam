use std::thread;
use std::time::Duration;

use kdam::{tqdm, MultiBar};

fn main() {
    let mut pb1 = tqdm!(total = 150);
    let mut pb2 = tqdm!(total = 100);
    let mut pb3 = tqdm!(total = 200);

    let mut bar_handle = MultiBar::new();
    bar_handle.append(&mut pb1);
    bar_handle.append(&mut pb2);
    bar_handle.append(&mut pb3);

    let thread1 = thread::spawn(move || {
        for _ in 0..150 {
            thread::sleep(Duration::from_secs_f32(0.1));
            pb1.update(1);
        }
    });

    let thread2 = thread::spawn(move || {
        for _ in 0..100 {
            thread::sleep(Duration::from_secs_f32(0.1));
            pb2.update(1);
        }
    });

    let thread3 = thread::spawn(move || {
        for _ in 0..200 {
            thread::sleep(Duration::from_secs_f32(0.1));
            pb3.update(1);
        }
    });

    // listen without blocking main thread
    thread::spawn(move || {
        bar_handle.listen();
    });

    // join other worker threads
    for thread in [thread1, thread2, thread3] {
        thread.join().unwrap();
    }
}

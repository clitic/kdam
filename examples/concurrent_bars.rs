use std::thread;
use std::time::Duration;
use kdam::{Bar, MultiBar};

fn main() {
    let mut pb1 = Bar {total: 150, ..Default::default()};
    let mut pb2 = Bar {total: 100, ..Default::default()};
    let mut pb3 = Bar {total: 200, ..Default::default()};
    
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

    let threads = vec![thread1, thread2, thread3];

    // listen without blocking main thread
    thread::spawn(move || {
        bar_handle.listen();
    });

    // join other worker threads
    for thread in threads {
        thread.join().unwrap();
    }
}

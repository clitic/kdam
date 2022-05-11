use std::thread;
use std::time::Duration;

use kdam::tqdm;

fn main() {
    let mut pb1 = tqdm!(total = 150, position = 0);
    let mut pb2 = tqdm!(total = 100, position = 1);
    let mut pb3 = tqdm!(total = 200, position = 2);

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

    // join other worker threads
    for thread in [thread1, thread2, thread3] {
        thread.join().unwrap();
    }

    eprint!("{}", "\n".repeat(3));
    println!("completed!");
}

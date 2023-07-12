use kdam::{tqdm, BarExt, RowManager};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    kdam::term::init(false);
    
    let mut manager = RowManager::new(3);

    for (i, total) in [150, 100, 200, 400, 500, 600].iter().enumerate() {
        manager.append(tqdm!(
            total = *total,
            leave = false,
            desc = format!("BAR {}", i),
            force_refresh = true
        ));
    }

    let manager = Arc::new(Mutex::new(manager));

    let manager1 = manager.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..150 {
            thread::sleep(Duration::from_secs_f32(0.02));
            let mut manager = manager1.lock().unwrap();
            manager.get_mut(0).unwrap().update(1);
            manager.notify(0);
        }
    });

    let manager2 = manager.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..100 {
            thread::sleep(Duration::from_secs_f32(0.02));
            let mut manager = manager2.lock().unwrap();
            manager.get_mut(1).unwrap().update(1);
            manager.notify(1);
        }
    });

    let manager3 = manager.clone();
    let thread3 = thread::spawn(move || {
        for _ in 0..200 {
            thread::sleep(Duration::from_secs_f32(0.02));
            let mut manager = manager3.lock().unwrap();
            manager.get_mut(2).unwrap().update(1);
            manager.notify(2);
        }
    });

    let manager4 = manager.clone();
    let thread4 = thread::spawn(move || {
        for _ in 0..400 {
            thread::sleep(Duration::from_secs_f32(0.02));
            let mut manager = manager4.lock().unwrap();
            manager.get_mut(3).unwrap().update(1);
            manager.notify(3);
        }
    });

    let manager5 = manager.clone();
    let thread5 = thread::spawn(move || {
        for _ in 0..500 {
            thread::sleep(Duration::from_secs_f32(0.02));
            let mut manager = manager5.lock().unwrap();
            manager.get_mut(4).unwrap().update(1);
            manager.notify(4);
        }
    });

    let manager6 = manager;
    let thread6 = thread::spawn(move || {
        for _ in 0..600 {
            thread::sleep(Duration::from_secs_f32(0.02));
            let mut manager = manager6.lock().unwrap();
            manager.get_mut(5).unwrap().update(1);
            manager.notify(5);
        }
    });

    for thread in [thread1, thread2, thread3, thread4, thread5, thread6] {
        thread.join().unwrap();
    }

    println!("\rcompleted!");
}

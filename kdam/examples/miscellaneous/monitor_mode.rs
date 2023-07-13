use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let pb = tqdm!(total = 100, force_refresh = true);
    let (pb_arc, monitor_thread) = kdam::monitor::bar(pb, 1.0);

    for _ in 0..100 {
        pb_arc.lock().unwrap().update(1)?;
        std::thread::sleep(std::time::Duration::from_secs_f32(3.0));
    }

    monitor_thread.join().unwrap();
    eprintln!();
    Ok(())
}

use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

use kdam::tqdm;

fn main() {
    let f = File::create("kdam-logs.txt").unwrap();
    for _ in tqdm!(0..100, file = Some(f)) {
        sleep(Duration::from_secs_f32(0.01));
    }
}

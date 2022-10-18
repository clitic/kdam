use kdam::tqdm;
use std::fs::File;

fn main() {
    let f = File::create("kdam-logs.txt").unwrap();

    for _ in tqdm!(0..100, file = Some(f)) {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.01));
    }
}

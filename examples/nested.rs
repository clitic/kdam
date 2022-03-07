use std::thread::sleep;
use std::time::Duration;

use kdam::tqdm;

fn main() {
    for _ in tqdm!(0..4, desc = "1st loop".to_string(), position = 0) {
        for _ in tqdm!(0..5, desc = "2nd loop".to_string(), position = 1) {
            for _ in tqdm!(0..50, desc = "3rd loop".to_string(), position = 2) {
                sleep(Duration::from_secs_f32(0.0001));
            }
        }
    }
    print!("{}", "\n".repeat(3));
    println!("completed!");
}

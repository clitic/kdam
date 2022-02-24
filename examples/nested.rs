use std::thread::sleep;
use std::time::Duration;

use kdam::term::move_up;
use kdam::tqdm;

fn main() {
    for _ in tqdm!(0..4, desc = "1st loop".to_string()) {
        println!();

        for _ in tqdm!(0..5, desc = "2nd loop".to_string()) {
            println!();

            for _ in tqdm!(0..50, desc = "3rd loop".to_string()) {
                sleep(Duration::from_secs_f32(0.0001));
            }
            move_up(1);
        }
        move_up(1);
    }
    print!("\n\n\n");
    println!("completed!");
}

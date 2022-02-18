use std::thread::sleep;
use std::time::Duration;

use kdam::{term::move_up, BarIter};

fn main() {
    let mut bar1 = (0..4).progress();
    bar1.set_description("1st loop", false);
    for _ in bar1 {
        println!(); // move cursor to newline for printing 2nd loop
        let mut bar2 = (0..5).progress();
        bar2.set_description("2st loop", false);

        for _ in bar2 {
            println!(); // move cursor to newline for printing 3rd loop
            let mut bar3 = (0..50).progress();
            bar3.set_description("3st loop", false);

            for _ in bar3 {
                sleep(Duration::from_secs_f32(0.0001));
            }
            move_up(1); // move cursor up for updating 2nd loop
        }
        move_up(1); // move cursor up for updating 1st loop
    }
    print!("\n\n\n"); // move cursor 3 lines down
    println!("completed!");
}

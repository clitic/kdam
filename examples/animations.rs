use std::thread;
use std::time::Duration;

use kdam::{tqdm, Animation};

fn main() {
    let render_length = 100;
    let sleep_time = 0.001;
    println!("animations:\n");

    for _ in tqdm!(0..render_length, desc = "tqdm".to_string()) {
        thread::sleep(Duration::from_secs_f32(sleep_time));
    }
    print!("\n\n");
    for _ in tqdm!(0..render_length, desc = "ascii".to_string(), ascii = true) {
        thread::sleep(Duration::from_secs_f32(sleep_time));
    }

    print!("\n\n");
    for _ in tqdm!(
        0..render_length,
        desc = "fillup".to_string(),
        animation = Animation::FillUp
    ) {
        thread::sleep(Duration::from_secs_f32(sleep_time));
    }

    print!("\n\n");
    for _ in tqdm!(
        0..render_length,
        desc = "classic".to_string(),
        animation = Animation::Classic
    ) {
        thread::sleep(Duration::from_secs_f32(sleep_time));
    }

    print!("\n\n");
    for _ in tqdm!(
        0..render_length,
        desc = "arrow".to_string(),
        animation = Animation::Arrow
    ) {
        thread::sleep(Duration::from_secs_f32(sleep_time));
    }
}

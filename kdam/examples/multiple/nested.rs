use kdam::tqdm;

fn main() {
    for _ in tqdm!(0..4, desc = "1st loop", position = 0) {
        for _ in tqdm!(0..5, desc = "2nd loop", position = 1) {
            for _ in tqdm!(0..50, desc = "3rd loop", position = 2) {
                std::thread::sleep(std::time::Duration::from_secs_f32(0.0001));
            }
        }
    }

    eprint!("{}", "\n".repeat(3));
    println!("completed!");
}

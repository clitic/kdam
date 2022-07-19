use kdam::prelude::*;

fn main() {
    let mut pb1 = tqdm!(total = 100, position = 0);
    let mut pb2 = tqdm!(total = 100, position = 4);

    kdam::write_at!(2, "any text can be placed {} bars", "between");

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb1.update(1);
        pb2.update(1);
    }

    eprint!("{}", "\n".repeat(5));
    println!("completed!");
}

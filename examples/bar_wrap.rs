use kdam::prelude::*;

fn main() {
    let mut pb = tqdm!(total = 10, wrap = true);
    pb.set_postfix("abcdefghijklmnopqrstuvwxyz0123456789");
    pb.refresh();

    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
        pb.update(1);
    }

    eprint!("\n");
}

use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 1000, mininterval = 0.0);
    for _ in 0..1000 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1);
    }
}

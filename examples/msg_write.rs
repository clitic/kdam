use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));

        pb.update(1);
        pb.write(format!("Done task {}", i));
    }
}

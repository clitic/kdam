use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 10);
    pb.refresh();

    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));

        pb.set_description(format!("GEN {}", i));
        pb.set_postfix(format!("str={}, lst={:?}", "h", [1, 2]));
        pb.update(1);
    }
}

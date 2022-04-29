use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 300, max_fps = true);
    pb.set_charset(&["\\", "|", "/", "-"]);

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1);
    }
}

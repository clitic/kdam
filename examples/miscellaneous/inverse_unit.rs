use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!(total = 300, force_refresh = true, inverse_unit = true);

    for i in 0..300 {
        if i > 5 {
            std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        } else {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }

        pb.update(1);
    }

    eprintln!();
}

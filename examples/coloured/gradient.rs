// [dependencies]
// kdam = { version = "0.2.7", features = ["gradient", "template"] }

use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!(
        total = 300,
        bar_format = "{animation} {percentage:3.0}%",
        colour = "gradient(#5A56E0,#EE6FF8)",
        force_refresh = true
    );

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1);
    }

    eprint!("\n");
}

// [dependencies]
// kdam = { version = "0.2.7", features = ["gradient", "template"] }

use kdam::term::Colorizer;
use kdam::tqdm;

fn main() {
    for _ in tqdm!(
        0..300,
        total = 300,
        bar_format = format!("{{animation}} {}", "{percentage:3.0}%".colorize("#EE6FF8")),
        colour = "gradient(#5A56E0,#EE6FF8)",
        force_refresh = true
    ) {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
    }

    eprintln!();
}

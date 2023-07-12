use kdam::{term::Colorizer, tqdm, Colour};
use std::io::{stderr, IsTerminal};

fn main() {
    kdam::term::init(stderr().is_terminal());

    for _ in tqdm!(
        0..300,
        total = 300,
        bar_format = format!("{{animation}} {}", "{percentage:3.0}%".colorize("#EE6FF8")),
        // You can also use "gradient(#5A56E0,#EE6FF8)".
        colour = Colour::gradient(&["#5A56E0", "#EE6FF8"]),
        force_refresh = true
    ) {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
    }

    eprintln!();
}

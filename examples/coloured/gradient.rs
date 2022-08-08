use kdam::formatx::Template;
use kdam::prelude::*;

fn main() {
    let mut pb = tqdm!(
        total = 300,
        bar_format = "{animation} {percentage}".parse::<Template>().unwrap(),
        colour = "gradient(#5A56E0,#EE6FF8)",
        force_refresh = true
    );

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1);
    }

    eprint!("\n");
}

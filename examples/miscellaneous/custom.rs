use kdam::{derive_bar_ext, tqdm, Bar, BarExt};

struct CustomBar {
    pb: Bar, // Required
}

fn render(progress: &mut CustomBar) -> String {
    format!(
        "Progress: {}/{}",
        progress.pb.fmt_counter(),
        progress.pb.fmt_total(),
    )
}

derive_bar_ext!(CustomBar, render);

fn main() {
    let mut pb = CustomBar {
        pb: tqdm!(total = 100, force_refresh = true),
    };

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1);
    }

    eprintln!();
}

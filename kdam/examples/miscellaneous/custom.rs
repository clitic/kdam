use kdam::{tqdm, Bar, BarExt};

#[derive(BarExt)]
struct CustomBar {
    #[bar]
    pb: Bar,
}

impl CustomBar {
    fn render(&mut self) -> String {
        format!(
            "Progress: {}/{}",
            self.pb.fmt_counter(),
            self.pb.fmt_total(),
        )
    }
}

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

use kdam::{tqdm, Bar, BarExt};
use std::{io::Result, num::NonZeroU16};

#[derive(BarExt)]
struct CustomBar {
    #[bar]
    pb: Bar,
}

impl CustomBar {
    /// Render progress bar text.
    fn render(&mut self) -> String {
        let fmt_percentage = self.pb.fmt_percentage(0);
        let padding = 1 + fmt_percentage.chars().count() as u16 + self.pb.animation.spaces() as u16;

        let ncols = self.pb.ncols_for_animation(padding);

        if ncols == 0 {
            self.pb.bar_length = padding - 1;
            fmt_percentage
        } else {
            self.pb.bar_length = padding + ncols;
            self.pb.animation.fmt_render(
                NonZeroU16::new(ncols).unwrap(),
                self.pb.percentage(),
                &None,
            ) + " "
                + &fmt_percentage
        }
    }
}

fn main() -> Result<()> {
    let mut pb = CustomBar {
        pb: tqdm!(total = 100, force_refresh = true),
    };

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1)?;
    }

    eprintln!();
    Ok(())
}

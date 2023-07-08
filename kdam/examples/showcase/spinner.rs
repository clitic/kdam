use kdam::{term::get_columns_or, Spinner};
use std::io::Write;

fn main() {
    let spin = Spinner::new(
        &[
            "▁▂▃",
            "▂▃▄",
            "▃▄▅",
            "▄▅▆",
            "▅▆▇",
            "▆▇█",
            "▇█▇",
            "█▇▆",
            "▇▆▅",
            "▆▅▄",
            "▅▄▃",
            "▄▃▂",
            "▃▂▁",
        ],
        30.0,
        1.0,
    );

    let mut stdout = std::io::stdout();
    let timer = std::time::Instant::now();

    loop {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        stdout
            .write_fmt(format_args!(
                "\r{}",
                spin.render_frames(timer.elapsed().as_secs_f32(), get_columns_or(3) as i16 / 3)
            ))
            .unwrap();
        stdout.flush().unwrap();
    }
}

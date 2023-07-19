use kdam::{term, term::Writer, Spinner};
use std::{
    num::NonZeroI16,
    time::{Duration, Instant},
};

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

    let timer = Instant::now();

    loop {
        std::thread::sleep(Duration::from_secs_f32(0.02));
        Writer::Stderr
            .print_at(
                0,
                spin.render_frames(
                    timer.elapsed().as_secs_f32(),
                    NonZeroI16::new((term::width().unwrap_or(30) / 3) as i16).unwrap(),
                )
                .as_bytes(),
            )
            .unwrap();
    }
}

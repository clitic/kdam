use kdam::prelude::*;

fn main() {
    let render_length = 300;

    let mut pb1 = tqdm!(
        total = render_length,
        desc = "tqdm    ",
        position = 0,
        force_refresh = true
    );
    let mut pb2 = tqdm!(
        total = render_length,
        desc = "ascii   ",
        animation = "ascii",
        position = 2,
        force_refresh = true
    );
    let mut pb3 = tqdm!(
        total = render_length,
        desc = "fillup  ",
        animation = "fillup",
        position = 4,
        force_refresh = true
    );
    let mut pb4 = tqdm!(
        total = render_length,
        desc = "classic ",
        animation = "classic",
        position = 6,
        force_refresh = true
    );
    let mut pb5 = tqdm!(
        total = render_length,
        desc = "arrow   ",
        animation = "arrow",
        position = 8,
        force_refresh = true
    );
    let mut pb6 = tqdm!(
        total = render_length,
        desc = "custom1 ",
        animation = kdam::Animation::custom(&["\\", "|", "/", "-"]),
        position = 10,
        force_refresh = true
    );
    let mut pb7 = tqdm!(
        total = render_length,
        desc = "custom2 ",
        animation = kdam::Animation::custom_with_fill(&["\\", "|", "/", "-"], "."),
        position = 12,
        force_refresh = true
    );

    println!("animations:\n");

    for _ in 0..render_length {
        pb1.update(1);
        pb2.update(1);
        pb3.update(1);
        pb4.update(1);
        pb5.update(1);
        pb6.update(1);
        pb7.update(1);
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
    }

    eprint!("{}", "\n".repeat(13));
}

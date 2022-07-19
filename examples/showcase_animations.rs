use kdam::prelude::*;
use kdam::Animation;

fn main() {
    let render_length = 300;

    let mut pb1 = tqdm!(
        total = render_length,
        desc = "tqdm    ".to_string(),
        position = 0,
        force_refresh = true
    );
    let mut pb2 = tqdm!(
        total = render_length,
        desc = "ascii   ".to_string(),
        animation = Animation::TqdmAscii,
        position = 2,
        force_refresh = true
    );
    let mut pb3 = tqdm!(
        total = render_length,
        desc = "fillup  ".to_string(),
        animation = Animation::FillUp,
        position = 4,
        force_refresh = true
    );
    let mut pb4 = tqdm!(
        total = render_length,
        desc = "classic ".to_string(),
        animation = Animation::Classic,
        position = 6,
        force_refresh = true
    );
    let mut pb5 = tqdm!(
        total = render_length,
        desc = "arrow   ".to_string(),
        animation = Animation::Arrow,
        position = 8,
        force_refresh = true
    );

    println!("animations:\n");

    for _ in 0..render_length {
        pb1.update(1);
        pb2.update(1);
        pb3.update(1);
        pb4.update(1);
        pb5.update(1);
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
    }

    eprint!("{}", "\n".repeat(9));
}

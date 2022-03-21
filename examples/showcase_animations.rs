use kdam::{tqdm, Animation};

fn main() {
    let render_length = 300;

    let mut pb1 = tqdm!(
        total = render_length,
        desc = "tqdm".to_string(),
        position = 0,
        max_fps = true
    );
    let mut pb2 = tqdm!(
        total = render_length,
        desc = "ascii".to_string(),
        ascii = true,
        position = 2,
        max_fps = true
    );
    let mut pb3 = tqdm!(
        total = render_length,
        desc = "fillup".to_string(),
        animation = Animation::FillUp,
        position = 4,
        max_fps = true
    );
    let mut pb4 = tqdm!(
        total = render_length,
        desc = "classic".to_string(),
        animation = Animation::Classic,
        position = 6,
        max_fps = true
    );
    let mut pb5 = tqdm!(
        total = render_length,
        desc = "arrow".to_string(),
        animation = Animation::Arrow,
        position = 8,
        max_fps = true
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

    print!("{}", "\n".repeat(9));
}

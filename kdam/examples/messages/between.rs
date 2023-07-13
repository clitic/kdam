use kdam::{term::Writer, tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    kdam::term::init(false);

    let mut pb1 = tqdm!(total = 100, position = 0);
    let mut pb2 = tqdm!(total = 100, position = 4);

    Writer::Stderr.print_at(2, "any text can be placed between bars".as_bytes())?;

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb1.update(1)?;
        pb2.update(1)?;
    }

    eprint!("{}", "\n".repeat(5));
    println!("completed!");

    Ok(())
}

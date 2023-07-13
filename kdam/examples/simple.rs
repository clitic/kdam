use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!(total = 100);

    for _ in 0..100 {
        pb.update(1)?;
    }

    eprintln!();
    Ok(())
}

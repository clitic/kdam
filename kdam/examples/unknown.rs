use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!();

    for _ in 0..10000000 {
        pb.update(1)?;
    }

    pb.refresh()?;
    eprintln!();

    Ok(())
}

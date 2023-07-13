use kdam::{tqdm, BarExt};
use std::io::Result;

fn main() -> Result<()> {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        if i == 5 && pb.input("Break Loop [y/n]: ")?.trim() == "y" {
            break;
        }

        pb.update(1)?;
    }

    eprintln!();
    Ok(())
}

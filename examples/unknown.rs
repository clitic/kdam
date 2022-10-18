use kdam::{tqdm, BarExt};

fn main() {
    let mut pb = tqdm!();

    for _ in 0..10000000 {
        pb.update(1);
    }
    pb.refresh();

    eprintln!();
}

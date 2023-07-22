use kdam::{rayon::prelude::*, TqdmIterator, TqdmParallelIterator};
use std::{time::{Instant, Duration}, thread::sleep};

fn main() {
    let now = Instant::now();
    (0..100).tqdm().for_each(|_| sleep(Duration::from_micros(1)));
    println!("\n\nstd: {} ms\n", now.elapsed().as_millis());

    let now = Instant::now();
    (0..100).into_par_iter().tqdm().for_each(|_| sleep(Duration::from_micros(1)));
    println!("\n\nrayon: {} ms", now.elapsed().as_millis());
}

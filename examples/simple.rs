use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 100);
    for _ in 0..100 {
        pb.update(1);
    }
}

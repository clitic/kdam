use kdam::tqdm;

fn main() {
    let mut pb = tqdm!(total = 100000000);

    for i in 0..100000000 {
        if i % 10000000 == 0 {
            pb.write(&format!(
                "reached at {}",
                kdam::format::format_sizeof(i, 1000)
            ));
        }
        pb.update(1);
    }
}

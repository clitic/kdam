use kdam::tqdm;

fn main() {
    let mut pb1 = tqdm!(total = 100, position = 0);
    let mut pb2 = tqdm!(total = 100, position = 4);

    pb1.write_at("any text can be placed in between bars".to_string(), 2);

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb1.update(1);
        pb2.update(1);
    }

    print!("{}", "\n".repeat(5));
    println!("completed!");
}

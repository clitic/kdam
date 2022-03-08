use std::time::Instant;

use kdam::tqdm;

static PYTHON_FILE: &str = r#"import tqdm

pb = tqdm.tqdm(total=100000, leave=False);

for _ in range(100000):
    pb.update(1)
"#;

fn bench_rust() -> f64 {
    let timer = Instant::now();
    let mut pb = tqdm!(total = 100000, leave = false);

    for _ in 0..100000 {
        pb.update(1);
    }
    timer.elapsed().as_secs_f64()
}

fn bench_python() -> f64 {
    let timer = Instant::now();

    std::process::Command::new("python")
        .args(&["-c", PYTHON_FILE])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    timer.elapsed().as_secs_f64()
}

fn average(results: Vec<f64>) -> f64 {
    let mut total = 0.0;
    for i in &results {
        total += i;
    }

    total / (results.len() as f64)
}

fn main() {
    let mut rust_results = vec![];
    let mut python_results = vec![];

    for _ in 0..10 {
        rust_results.push(bench_rust());
        python_results.push(bench_python());
    }

    println!(
        "results (lower is better):\nrust: {}\npython: {}",
        average(rust_results),
        average(python_results)
    );
}

// [dependencies]
// kdam = { version = "0.3.0", features = ["writer"] }

use kdam::{tqdm, BarExt};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut pb = tqdm!(total = 100);
    let mut f = File::create("kdam-logs.txt").unwrap();
    f.write_all("Writing Logs\n".as_bytes()).unwrap();

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.01));
        pb.update_writer(1, &mut f);
    }

    f.write_all("Finished Writing Logs\n".as_bytes()).unwrap();
}

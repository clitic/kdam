use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use kdam::Bar;

fn main() {
    let src_file_path = "Cargo.toml";
    let dst_file_path = "Copy Of Cargo.toml";

    let src_file = File::open(src_file_path).unwrap();
    let mut dst_file = File::create(dst_file_path).unwrap();

    let mut reader = BufReader::new(src_file);
    let chunk_size = 1024;

    let mut pb = Bar {
        total: std::fs::metadata(src_file_path).unwrap().len(),
        unit_scale: true,
        unit_divisor: 1024,
        unit: "B".to_string(),
        ..Default::default()
    };

    loop {
        let mut chunk = vec![];
        let reader_ref = reader.by_ref();
        reader_ref.take(chunk_size).read_to_end(&mut chunk).unwrap();
        let chunk_len = chunk.len();
        dst_file.write(&chunk).unwrap();
        pb.update(chunk_len as u64);

        if chunk_len == 0 {
            pb.refresh();
            break;
        }
    }
}

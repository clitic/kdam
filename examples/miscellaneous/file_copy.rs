use kdam::{tqdm, BarExt};
use std::fs::File;
use std::io::{BufReader, Read, Result, Write};

fn main() -> Result<()> {
    let src_file_path = "Cargo.toml"; // replace this with any big size file
    let dst_file_path = "Cargo (Copy).toml";

    let src_file = File::open(src_file_path)?;
    let mut dst_file = File::create(dst_file_path)?;

    let mut reader = BufReader::new(src_file);
    let chunk_size = 1024;

    let mut pb = tqdm!(
        total = std::fs::metadata(src_file_path)?.len() as usize,
        unit_scale = true,
        unit_divisor = 1024,
        unit = "B"
    );

    loop {
        let mut chunk = vec![];
        reader.by_ref().take(chunk_size).read_to_end(&mut chunk)?;
        let chunk_len = chunk.len();
        dst_file.write_all(&chunk)?;
        pb.update(chunk_len);

        if chunk_len == 0 {
            pb.refresh();
            eprintln!();
            break;
        }
    }

    Ok(())
}

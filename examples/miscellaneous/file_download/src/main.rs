use futures_util::StreamExt;
use kdam::prelude::*;
use std::io::Write;

const URL: &str = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe";
const PATH: &str = "rustup-init.exe";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::Client::new().get(URL).send().await?;

    let mut pb = tqdm!(
        total = res.content_length().unwrap_or(0) as usize,
        unit_scale = true,
        unit_divisor = 1024,
        unit = "B",
        force_refresh = true
    );

    pb.write(format!("Downloading {}", URL));

    let mut file = std::fs::File::create(PATH)?;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write(&chunk)?;
        pb.update(chunk.len());
    }

    pb.refresh();
    println!("\nDownloaded {} to {}", URL, PATH);
    Ok(())
}

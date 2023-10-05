use futures_util::StreamExt;
use kdam::{tqdm, BarExt};
use reqwest::Client;
use std::{error::Error, fs::File, io::Write};

const URL: &str = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe";
const PATH: &str = "rustup-init.exe";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let res = Client::new().get(URL).send().await?;

    let mut pb = tqdm!(
        total = res.content_length().unwrap_or(0) as usize,
        unit_scale = true,
        unit_divisor = 1024,
        unit = "B",
        force_refresh = true
    );

    pb.write(format!("Downloading {}", URL))?;

    let mut file = File::create(PATH)?;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        pb.update(chunk.len())?;
    }

    pb.refresh()?;
    eprintln!("\nDownloaded {}", PATH);
    Ok(())
}

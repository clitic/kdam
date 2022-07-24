// original gist: https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d

use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use kdam::prelude::*;
use reqwest::Client;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let mut pb = tqdm!(
        total = total_size as usize,
        unit_scale = true,
        unit_divisor = 1024,
        unit = "B",
        force_refresh = true
    );

    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut stream = res.bytes_stream();

    println!("Downloading {}", url);
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        pb.update(chunk.len());
    }
    pb.refresh();
    println!("\nDownloaded {} to {}", url, path);
    return Ok(());
}

#[tokio::main]
async fn main() {
    download_file(
        &Client::new(),
        "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe",
        "rustup-init.exe",
    )
    .await
    .unwrap();
}

// original gist: https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d

use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use kdam::Bar;
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

    let mut pb = Bar {
        total: total_size,
        unit_scale: true,
        unit_divisor: 1024,
        unit: "B".to_string(),
        ..Default::default()
    };

    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut stream = res.bytes_stream();

    println!("Downloading {}", url);
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        pb.update(chunk.len() as u64);
    }
    pb.refresh();
    println!("\nDownloaded {} to {}", url, path);
    return Ok(());
}

#[tokio::main]
async fn main() {
    download_file(
        &Client::new(),
        "https://file-examples-com.github.io/uploads/2017/04/file_example_MP4_1920_18MG.mp4",
        "file_example_MP4_1920_18MG.mp4",
    )
    .await
    .unwrap();
}

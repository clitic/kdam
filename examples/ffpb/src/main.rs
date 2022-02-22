// original source: https://github.com/althonos/ffpb/blob/master/ffpb.py

use kdam::Bar;
use regex::Regex;
use std::io::Read;
use std::io::Write;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let duration_rx = Regex::new(r"Duration: (\d{2}):(\d{2}):(\d{2})\.\d{2}").unwrap();
    let progress_rx = Regex::new(r"time=(\d{2}):(\d{2}):(\d{2})\.\d{2}").unwrap();
    let fps_rx = Regex::new(r"(\d{2}\.\d{2}|\d{2}) fps").unwrap();
    let args: Vec<String> = std::env::args().collect();

    if args[1..].len() == 0 {
        println!("a progress bar for ffmpeg.\n");
        println!("usage:\n  ffpb [ffmpeg <options>]\n");
        println!("examples:\n  ffpb -i test.mkv test.mp4\n  ffpb -i test.mkv -c:v copy test.mp4");
        return Ok(());
    }

    let ffmpeg = Command::new("ffmpeg")
        .args(&args[1..])
        // .args(["-i", "D:\\coding\\Rust.mp4", "-c", "copy", "rust.mkv"])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let ffmpeg_stderr = ffmpeg
        .stderr
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard error."))?;

    let mut reader = BufReader::new(ffmpeg_stderr);

    let mut pb = Bar {
        dynamic_ncols: true,
        ..Default::default()
    };

    let mut duration = None;
    let mut fps = None;
    let mut check_overwrite = true;

    let mut read_byte;
    if cfg!(target_os = "windows") {
        read_byte = b'\r';
    } else {
        read_byte = b'\n';
    }

    loop {
        let mut prepend_text = String::from("");

        if check_overwrite {
            let mut pre_buf = [0; 6];
            reader.read_exact(&mut pre_buf).unwrap();

            prepend_text.push_str(&String::from_utf8_lossy(&pre_buf));

            if prepend_text.contains("File ") {
                let mut post_buf = vec![];
                reader.read_until(b']', &mut post_buf).unwrap();
                print!("File {} ", String::from_utf8(post_buf).unwrap());
                std::io::stdout().flush().unwrap();
                check_overwrite = false;
                read_byte = b'\r';
            } else if prepend_text.starts_with("\nframe=") || prepend_text.starts_with("frame=") {
                check_overwrite = false;
                read_byte = b'\r';
            }

            if pb.i != 0 {
                check_overwrite = false;
                read_byte = b'\r';
            }
        }
        let mut buf = vec![];
        reader.read_until(read_byte, &mut buf).unwrap();
        let line = String::from_utf8(buf);
        if line.is_ok() {
            let std_line = prepend_text + &line.unwrap();

            if std_line == "" {
                pb.refresh();
                println!("");
                break;
            }

            if duration.is_none() {
                if let Some(x) = duration_rx.captures_iter(&std_line).next() {
                    let hours = x.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let minutes = x.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    let seconds = x.get(3).unwrap().as_str().parse::<u64>().unwrap();
                    duration = Some((((hours * 60) + minutes) * 60) + seconds);
                    pb.total = duration.unwrap();
                }
            }
            if fps.is_none() {
                if let Some(y) = fps_rx.captures_iter(&std_line).next() {
                    fps = Some(y.get(1).unwrap().as_str().parse::<f32>().unwrap() as u64);
                    pb.unit = " frame".to_string();
                }
            }

            if let Some(x) = progress_rx.captures_iter(&std_line).next() {
                let hours = x.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let minutes = x.get(2).unwrap().as_str().parse::<u64>().unwrap();
                let seconds = x.get(3).unwrap().as_str().parse::<u64>().unwrap();
                let mut current = (((hours * 60) + minutes) * 60) + seconds;

                if fps.is_some() {
                    current *= fps.unwrap();

                    if pb.total == duration.unwrap_or(0) {
                        pb.total *= fps.unwrap();
                    }
                } else {
                    pb.unit = " second".to_string();
                }

                if current as isize - pb.i as isize > 0 {
                    pb.update(current - pb.i);
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}

use kdam::prelude::*;
use kdam::{Column, RichProgress};

fn main() {
    let mut pb = RichProgress::new(
        tqdm!(
            total = 231231231,
            unit_scale = true,
            unit_divisor = 1024,
            unit = "B"
        ),
        vec![
            Column::Spinner(
                "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                80.0,
                1.0,
            ),
            Column::text("[bold blue]?"),
            Column::Bar,
            Column::Percentage(1),
            Column::text("•"),
            Column::CountTotal,
            Column::text("•"),
            Column::Rate,
            Column::text("•"),
            Column::RemainingTime,
        ],
    );

    pb.write("download will begin in 5 seconds".colorize("bold red"));

    while pb.pb.elapsed_time() <= 5.0 {
        pb.refresh();
    }

    pb.replace(1, Column::text("[bold blue]docker.exe"));
    pb.write("downloading docker.exe".colorize("bold cyan"));

    let total_size = 231231231;
    let mut downloaded = 0;

    while downloaded < total_size {
        let new = std::cmp::min(downloaded + 223211, total_size);
        downloaded = new;
        pb.update_to(new);
        std::thread::sleep(std::time::Duration::from_millis(12));
    }

    pb.write("downloaded docker.exe".colorize("bold green"));
    eprintln!();
}

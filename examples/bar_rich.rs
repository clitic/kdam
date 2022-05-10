use kdam::{tqdm, RichProgress, Column};
use kdam::term::{Colorizer, Output};

fn main() {
    let mut pb = RichProgress::new(tqdm!(
        total = 231231231,
        unit_scale = true,
        unit_divisor = 1024,
        unit = "B".to_string()
    ));

    pb.add(Column::Spinner(vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"].iter().map(|x| {x.to_string()}).collect(), 80.0, 1.0));
    pb.add(Column::Text("ubuntu-20.04-desktop-amd64.iso".to_string(), Some("bold blue".to_string())));
    pb.add(Column::Bar);
	pb.add(Column::Percentage(1));
    pb.add(Column::Text("•".to_string(), None));
    pb.add(Column::CountTotal);
    pb.add(Column::Text("•".to_string(), None));
    pb.add(Column::Rate);
    pb.add(Column::Text("•".to_string(), None));
    pb.add(Column::RemainingTime);

    pb.write("downloading ubuntu-20.04-desktop-amd64.iso".colorize("bold cyan"));

    let total_size = 231231231;
    let mut downloaded = 0;

    while downloaded < total_size {
        let new = std::cmp::min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        std::thread::sleep(std::time::Duration::from_millis(12));
    }

    pb.write("downloaded ubuntu-20.04-desktop-amd64.iso".colorize("bold green"));
    kdam::finish(1, Output::Stderr);
}

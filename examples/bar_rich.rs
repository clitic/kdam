use kdam::{tqdm, RichProgress, Column};

fn main() {
    let mut pb = RichProgress::new(tqdm!(
        total = 300000000,
        unit_scale = true,
        unit_divisor = 1024,
        unit = "B".to_string()
    ));

    // sd.add(kdam::rich::Column::Spinner(vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"].iter().map(|x| {x.to_string()}).collect(), 80.0, 1.0));
    pb.add(Column::Text("ubuntu-20.04-desktop-amd64.iso".to_string(), Some("bold blue".to_string())));
    pb.add(Column::Bar);
	pb.add(Column::TaskProgress(1));
    pb.add(Column::Text("•".to_string(), None));
    pb.add(Column::Download);
    pb.add(Column::Text("•".to_string(), None));
    pb.add(Column::TransferSpeed);
    pb.add(Column::Text("•".to_string(), None));
    pb.add(Column::TimeRemaining);

    // let mut downloaded = 0;
    // let total_size = 231231231;

    // while downloaded < total_size {
    //     let new = std::cmp::min(downloaded + 223211, total_size);
    //     downloaded = new;
    //     pb.set_position(new);
    //     std::thread::sleep(std::time::Duration::from_millis(12));
    // }

    // kdam::finish(1, kdam::Output::Stderr);
    // println!("downloaded");

    for _ in 0..300000000 {
        pb.update(1);
    }
}

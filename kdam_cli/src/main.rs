// use clap::Parser;

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[clap(short, long)]
//     name: String,

//     /// Number of times to greet
//     #[clap(short, long, default_value_t = 1)]
//     count: u8,
// }

// fn main() {
//     let args = Args::parse();

//     for _ in 0..args.count {
//         println!("Hello {}!", args.name)
//     }
// }

// pub fn set_bytes_mode(&mut self) {
//     self.unit_scale = true;
//     self.unit_divisor = 1024;
//     self.unit = "B".to_string();
// }

use std::io::{self, BufRead};
use kdam::Bar;

fn main() {
    let stdin = io::stdin();

    let args: Vec<String> = std::env::args().collect();

    let mut pb = Bar {total: args[1].parse::<usize>().unwrap_or(0), ..Default::default()};


    for line in stdin.lock().lines() {
        let std_line = line.unwrap_or("0".to_string());
        let postion = std_line.parse::<usize>().unwrap_or(0);

        // if postion == 0 {
            // }
            // println!("{}", std_line);

        if postion as isize - pb.i as isize >= 0 {
            pb.update(postion - pb.i);
        }

    }
    pb.refresh();
    println!("");
}
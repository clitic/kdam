use kdam::prelude::*;

fn main() {
    let mut pb = tqdm!(total = 10);

    for i in 0..10 {
        if i == 5 {
            if pb.input("Break Loop [y/n]: ").unwrap().trim() == "y" {
                break;
            }
        }

        pb.update(1);
    }

    eprint!("\n");
}

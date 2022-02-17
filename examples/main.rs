// mod pbcook;
// use std::thread::sleep;
// use std::time::Duration;

fn main() {
    // println!("{}sas{}", pbcook::term::colour("#008080"), pbcook::term::COLOUR_RESET);
    // let mut bar = pbcook::Bar::new(100000000);
    
    // // bar.set_colour("#008080");
    // // bar.set_file(std::fs::File::options().write(true).open("foo.txt").expect("adad"));
    // // bar.set_postfix("190113", false);
    // for i in 0..100000000 {
    //     // sleep(Duration::from_secs_f32(0.1));
    //     if i % 10000000 == 0 {
    //         bar.write(&format!("reached at {}", pbcook::fmt_data::format_sizeof(i, 1000)));
    //     }
    //     bar.update(1);
    // }
    // bar.refresh();

    let kl = kdam::Bar::from_iterator(0..10000000);
    // kl.set_colour("red");
    for _ in kl {

    }

}
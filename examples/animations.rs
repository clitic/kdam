use kdam::{Bar, Animation};

fn main() {
    let render_length = 10000000;
    println!("animations:\n");
    
    let mut pb = Bar::from_iterator(0..render_length);
    pb.set_description("tqdm", false);
    for _ in pb {}
    
    
    print!("\n\n");
    pb = Bar::from_iterator(0..render_length);
    pb.set_description("ascii", false);
    pb.set_animation(Animation::TqdmAscii);
    for _ in pb {}

    print!("\n\n");
    pb = Bar::from_iterator(0..render_length);
    pb.set_description("fillup", false);
    pb.set_animation(Animation::FillUp);
    for _ in pb {}

    print!("\n\n");
    pb = Bar::from_iterator(0..render_length);
    pb.set_description("classic", false);
    pb.set_animation(Animation::Classic);
    for _ in pb {}

    print!("\n\n");
    pb = Bar::from_iterator(0..render_length);
    pb.set_description("arrow", false);
    pb.set_animation(Animation::Arrow);
    for _ in pb {}
}

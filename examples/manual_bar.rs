use kdam::Bar;

fn main() {
    let mut pb = Bar::new(100);
    
    for _ in 0..100 {
        pb.update(1);
    }
}

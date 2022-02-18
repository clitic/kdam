use kdam::BarIter;

fn main() {
    let pb = ["Earth", "Mars", "Saturn", "Jupiter"].iter().progress();

    for _ in pb {
    }
}

use automata::ca;
use std::time::Instant;

fn main() {
    println!("Generating layers... ");
    let start = Instant::now();
    ca::iter_layers(30).skip(50000).next().unwrap();
    let done = Instant::now();
    let duration = done.duration_since(start);
    println!("Done in {:?}", duration);
}

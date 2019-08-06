extern crate bitvec;
pub mod ca;

use bitvec::prelude::*;
use ca::*;
use std::str;

fn main() {
    print_rule(30, 30);
}

fn print_rule(rule: u8, length: usize) {
    let mut layers = iter_layers(rule);

    for i in 0..length {
        // {' ' * length-i}{layer}
        let pad = str::repeat(" ", length - i);
        let layer = bitvec_string(&layers.next().unwrap());
        println!("{}{}", pad, layer);
    }
}

fn bitvec_string(vec: &BitVec) -> String {
    vec.iter()
        .map(|cell| if cell { "#" } else { " " })
        .collect::<String>()
}

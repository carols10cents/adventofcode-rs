use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./src/input.txt")
                        .expect("Could not open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input.txt");
}

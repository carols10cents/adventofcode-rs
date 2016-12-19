use std::fs::File;
use std::io::prelude::*;

extern crate advent;

fn main() {
    let mut file = File::open("./src/input.txt")
                        .expect("Could not open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input.txt");

    let answer = advent::puzzle(&input)
                        .expect("Could not figure it out");
    println!("The answer is {}", answer);
}

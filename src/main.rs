use std::fs::File;
use std::io::prelude::*;

extern crate advent;

use advent::Keypad;

fn main() {
    let keypad_input = [
        "  1  ",
        " 234 ",
        "56789",
        " ABC ",
        "  D  ",
    ].join("\n");
    let mut keypad = Keypad::new(&keypad_input);

    let mut file = File::open("./src/input.txt")
                        .expect("Could not open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read input.txt");

    let answer = advent::puzzle(&mut keypad, &input);
    println!("The answer is {}", answer);
}

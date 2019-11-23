extern crate day_4;
extern crate utils;

use std::env;

fn main() {
    let input_file = env::args()
        .nth(1)
        .expect("Pass the input file as first parameter");

    println!("==== [AOC] Day 4 ====");
    println!("Reading data from {}", input_file);

    let data = utils::load_file(&input_file).expect("Couldn't read input file");
    let lines: Vec<&str> = data.lines().collect();
    println!("--- Part 1 ---");
    println!("Result: {}", day_4::part_1(lines));

    let lines: Vec<&str> = data.lines().collect();
    println!("--- Part 2 ---");
    println!("Result: {}", day_4::part_2(lines));
}

use aoc_2023::get_calibration_sum;
use std::fs;

fn main() {
    println!("Hello, world!");
    let lines = fs::read_to_string("data/day-01.txt").unwrap();
    let sum = get_calibration_sum(&lines);
    println!("sum {}", sum)
}

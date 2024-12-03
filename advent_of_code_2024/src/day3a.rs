use std::fs;
use regex::Regex;

pub fn run() {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut mul_sum: i32 = 0;

    for line in fs::read_to_string("inputs/day3.txt").unwrap().lines() {
        for (_, [val1, val2]) in regex.captures_iter(line).map(|m| m.extract()) {
            mul_sum += val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap();
        }
    }

    println!("Sum of multiplications: {mul_sum}")
}

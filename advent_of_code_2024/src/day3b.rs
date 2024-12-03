use std::fs;
use regex::Regex;

pub fn run() {
    let statement_regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut mul_sum: i32 = 0;
    let mut mul_enabled = true;

    for line in fs::read_to_string("inputs/day3.txt").unwrap().lines() {
        for (_, [statement]) in statement_regex.captures_iter(line).map(|m| m.extract()) {
            if statement == "do()" {
                mul_enabled = true;
            } else if statement == "don't()" {
                mul_enabled = false;
            } else if mul_enabled {
                let mul_vals = mul_regex.captures(statement).unwrap();
                mul_sum += mul_vals.get(1).unwrap().as_str().parse::<i32>().unwrap() * mul_vals.get(2).unwrap().as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("Sum of multiplications: {mul_sum}");
}

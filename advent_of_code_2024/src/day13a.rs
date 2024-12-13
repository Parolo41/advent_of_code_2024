use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

pub fn run() {
    let mut machines = Vec::new();

    let input = fs::read_to_string("inputs/day13.txt").unwrap();
    let mut input_lines = input.lines();

    while let Some(button_a_string) = input_lines.next() {
        let button_a_regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let button_a_matches = button_a_regex.captures(button_a_string).unwrap();

        let button_b_string = input_lines.next().unwrap();
        let button_b_regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let button_b_matches = button_b_regex.captures(button_b_string).unwrap();

        let prize_string = input_lines.next().unwrap();
        let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let prize_matches = prize_regex.captures(prize_string).unwrap();

        machines.push(Machine {
            button_a: (
                button_a_matches[1].parse().unwrap(), 
                button_a_matches[2].parse().unwrap()
            ),
            button_b: (
                button_b_matches[1].parse().unwrap(), 
                button_b_matches[2].parse().unwrap()
            ),
            prize: (
                prize_matches[1].parse().unwrap(), 
                prize_matches[2].parse().unwrap()
            ),
        });

        input_lines.next();
    }

    let mut cost  = 0;

    for machine in machines {
        for i in (0..=100).rev() {
            let x = machine.prize.0 - (machine.button_b.0 * i);
            let y = machine.prize.1 - (machine.button_b.1 * i);

            if x >= 0
            && y >= 0
            && x % machine.button_a.0 == 0
            && y % machine.button_a.1 == 0
            && x / machine.button_a.0 == y / machine.button_a.1 {
                cost += i + ((x / machine.button_a.0) * 3);
            }
        }
    }

    println!("{}", cost);
}
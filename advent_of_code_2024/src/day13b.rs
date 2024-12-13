use std::{cmp, fs, ops::Neg};

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
                prize_matches[1].parse::<i64>().unwrap() + 10000000000000, 
                prize_matches[2].parse::<i64>().unwrap() + 10000000000000
            ),
        });

        input_lines.next();
    }

    let mut cost  = 0;

    for machine in machines {
        println!("Machine started {:#?}", machine);

        let a = machine.button_a;
        let b = machine.button_b;
        let p = machine.prize;

        let p_rat = ratio(p);
        let a_rat = ratio(a);
        let b_rat = ratio(b);
        println!("Equal ratios {a_rat} {b_rat} {p_rat}");

        if a_rat == p_rat {
            println!("Equal ratios {a_rat} {b_rat} {p_rat}");
            
            if p.0 % a.0 == 0 {
                cost += p.0 / a.0;
            } else {
                continue;
            }
        }

        if b_rat == p_rat {
            println!("Equal ratios {a_rat} {b_rat} {p_rat}");
            
            if p.0 % b.0 == 0 {
                cost += p.0 / b.0;
            } else {
                continue;
            }
        }

        if (a_rat > p_rat) == (b_rat > p_rat) {
            println!("Skipped");
            continue;
        }

        let mut seek_factor: i64 = 1_000_000_000;
        let mut cursor: (i64, i64) = (0, 0);
        let mut prev_rat = p_rat;

        loop {
            if (ratio(diff(p, cursor)) > b_rat) != (prev_rat > b_rat)
            || cursor.0 > p.0
            || cursor.1 > p.1 {
                if seek_factor == 1 {
                    println!("Not solved");
                    break;
                }

                cursor = add(cursor, a, seek_factor, -1);

                seek_factor /= 1000;
                prev_rat = ratio(diff(p, cursor));

                println!("{prev_rat}");
            }

            cursor = add(cursor, a, seek_factor, 1);

            if seek_factor == 1 {
                let diff = diff(p, cursor);

                if diff.0 % b.0 == 0 
                && diff.1 % b.1 == 0
                && diff.0 / b.0 == diff.1 / b.1 {
                    cost += (cursor.0 / a.0 * 3) + (diff.0 / b.0);
                    println!("Solved at {cursor:?}");
                    break;
                }
            }
        }
    }

    println!("{}", cost);
}

fn ratio(vector: (i64, i64)) -> f64 {
    vector.0 as f64 / vector.1 as f64
}

fn add(vector1: (i64, i64), vector2: (i64, i64), vector2_factor: i64, direction: i64) -> (i64, i64) {
    (vector1.0 + (vector2.0 * vector2_factor * direction), vector1.1 + (vector2.1 * vector2_factor * direction))
}

fn diff(vector1: (i64, i64), vector2: (i64, i64)) -> (i64, i64) {
    (vector1.0 - vector2.0, vector1.1 - vector2.1)
}
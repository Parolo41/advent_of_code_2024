use std::fs;

pub fn run() {
    let mut safe_count = 0;

    for line in fs::read_to_string("inputs/day2.txt").unwrap().lines() {
        let values = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        safe_count += is_safe(values) as i32
    }

    println!("Safe report count: {safe_count}");
}

fn is_safe(values: Vec<i32>) -> bool {
    let increasing = values[0] < values[1];

    for i in 1..values.len() {
        if increasing && values[i-1] >= values[i] { return false; }
        if !increasing && values[i-1] <= values[i] { return false; }
        if values[i-1].abs_diff(values[i]) > 3 { return false; }
    }

    return true;
}

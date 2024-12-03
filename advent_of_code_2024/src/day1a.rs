use std::fs;

pub fn run() {
    let mut diff_sum : u32 = 0;

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in fs::read_to_string("inputs/day1.txt").unwrap().lines() {
        let values = line.split_whitespace().collect::<Vec<&str>>();

        if values.len() != 2 {
            println!("Incorrect size of line: {line}");
            break;
        }

        left_values.push(values[0].parse::<u32>().unwrap());
        right_values.push(values[1].parse::<u32>().unwrap());
    }

    left_values.sort();
    right_values.sort();

    for (i, left_val) in left_values.iter().enumerate() {
        diff_sum += left_val.abs_diff(right_values[i]);
    }
    
    println!("Sum of differences: {diff_sum}");
}

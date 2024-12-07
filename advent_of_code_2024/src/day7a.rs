use std::fs;

pub fn run() {
    let mut valid_calib_sum = 0;

    'line_read: for line in fs::read_to_string("inputs/day7.txt").unwrap().lines() {
        let split_vals = line.split(":").collect::<Vec<&str>>();

        println!("{line:?}");

        let calib_result = split_vals[0].parse::<u64>().expect("Operation result should be an integer");
        
        let calib_vals = split_vals[1].trim().split(" ").map(|v| v.parse().expect("Operation values should be integers")).collect::<Vec<u64>>();

        let possible_swaps = 2_usize.pow((calib_vals.len() - 1) as u32);

        for i in 0..possible_swaps {
            let mut running_total = calib_vals[0] as u64;
            let mut operator_bit_val = 1;

            for val in calib_vals[1..].iter() {
                if operator_bit_val & i > 0 {
                    running_total *= val;
                } else {
                    running_total += val;
                }

                operator_bit_val *= 2;
            }

            if running_total == calib_result {
                valid_calib_sum += calib_result;
                println!("Valid");
                continue 'line_read;
            }
        }
    }

    println!("Sum of calibrations: {valid_calib_sum}");
}

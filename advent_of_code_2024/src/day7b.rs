use std::fs;

pub fn run() {
    let mut valid_calib_sum = 0;

    'line_read: for line in fs::read_to_string("inputs/day7.txt").unwrap().lines() {
        let split_vals = line.split(":").collect::<Vec<&str>>();

        let calib_result = split_vals[0].parse::<u64>().expect("Operation result should be an integer");
        
        let calib_vals = split_vals[1].trim().split(" ").map(|v| v.parse().expect("Operation values should be integers")).collect::<Vec<u64>>();

        let possible_swaps = 3_usize.pow((calib_vals.len() - 1) as u32);

        'swap_attempt: for i in 0..possible_swaps {
            let mut running_total = calib_vals[0] as u64;
            let mut swap_tracker = i;

            for val in calib_vals[1..].iter() {
                let op_index = swap_tracker % 3;

                match op_index {
                    0 => running_total += val,
                    1 => running_total *= val,
                    2 => running_total = {
                        let log = val.ilog10();
                        let digit_mul = 10u64.pow(log + 1);
                        running_total * digit_mul + val
                    },
                    _ => panic!("Invalid op index: {op_index}"),
                }

                if running_total > calib_result { continue 'swap_attempt; }

                swap_tracker /= 3;
            }

            if running_total == calib_result {
                valid_calib_sum += calib_result;
                continue 'line_read;
            }
        }
    }

    println!("Sum of calibrations: {valid_calib_sum}");
}

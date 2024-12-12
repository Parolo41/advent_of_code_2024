use std::fs;

pub fn run() {
    let mut stones = Vec::new();

    for line in fs::read_to_string("inputs/day11.txt").unwrap().lines() {
        stones = line.split(" ").map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    }

    let loops = 25;

    for _ in 0..loops {
        let mut temp_stones = stones.clone();
        let mut index_shift = 0;

        for (i, stone) in stones.iter().enumerate() {
            if *stone == 0 {
                temp_stones[i + index_shift] = 1;
            } else {
                let log10 = stone.ilog10();

                if log10 % 2 == 1 {
                    let factor = 10_u64.pow((log10 + 1) / 2);
                    temp_stones.insert(i + index_shift + 1, stone % factor);
                    temp_stones[i + index_shift] = stone / factor;

                    index_shift += 1;
                } else {
                    temp_stones[i + index_shift] = stone * 2024;
                }
            }
        }

        stones = temp_stones;
    }

    println!("Number of stones: {}", stones.len());
}
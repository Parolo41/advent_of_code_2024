use std::{collections::HashMap, fs};

pub fn run() {
    let mut stone_map = HashMap::new();

    for line in fs::read_to_string("inputs/day11.txt").unwrap().lines() {
        for stone in line.split(" ") {
            stone_map.insert(stone.parse::<u64>().unwrap(), 1_u64);
        }
    }

    for _ in 0..75 {
        let mut stone_map_copy: HashMap<u64, u64> = HashMap::new();

        for (stone, count) in stone_map.iter() {
            if *stone == 0 {
                let new_count = stone_map_copy.entry(1).or_insert(0);
                *new_count += count;
            } else {
                let log = stone.ilog10();

                if log % 2 == 1 {
                    let factor = 10_u64.pow((log + 1) / 2);

                    let new_count = stone_map_copy.entry(stone % factor).or_insert(0);
                    *new_count += count;
                    
                    let new_count = stone_map_copy.entry(stone / factor).or_insert(0);
                    *new_count += count;
                } else {
                    let new_count = stone_map_copy.entry(stone * 2024).or_insert(0);
                    *new_count += count;
                }
            }
        }

        stone_map = stone_map_copy;
    }

    let mut stone_count = 0;

    for count in stone_map.into_values() {
        stone_count += count;
    }

    println!("Stone count: {stone_count}");
}
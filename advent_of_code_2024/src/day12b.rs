use std::{collections::HashMap, fs};

#[derive(PartialEq, Clone, Copy, Debug)]
struct Coords { x: usize, y: usize }

impl Coords {
    fn next_to(&self, other: Coords) -> bool {
        (self.x == other.x || self.y == other.y)
        && (self.x.abs_diff(other.x) == 1 || self.y.abs_diff(other.y) == 1)
    }
}

pub fn run() {
    let mut crops = HashMap::new();

    for (y, line) in fs::read_to_string("inputs/day12.txt").unwrap().lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            let crop_coords = crops.entry(byte).or_insert(Vec::new());
            crop_coords.push(Coords {x, y});
        }
    }

    let mut regions = Vec::new();

    for mut crop_coords in crops.into_values() {
        while let Some(start_coord) = crop_coords.pop() {
            let mut adjacent_coords = Vec::new();
            let mut to_check_coords = Vec::new();
            to_check_coords.push(start_coord);

            while let Some(coord) = to_check_coords.pop() {
                let (mut adjacent, remaining): (Vec<_>, Vec<_>) = crop_coords.clone().into_iter().partition(|&c| {
                    coord.next_to(c) && !adjacent_coords.contains(&c) && !to_check_coords.contains(&c)
                });

                to_check_coords.append(&mut adjacent);
                crop_coords = remaining;
                adjacent_coords.push(coord);
            }

            regions.push(adjacent_coords);
        }
    }

    let mut price_sum = 0;

    for region in regions {
        price_sum += count_sides(&region) * region.len() as u32;
    }

    println!("Total cost: {price_sum}");
}

fn count_sides(region: &Vec<Coords>) -> u32 {
    let mut border_sides = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    for coords in region {
        let neighbors = [
            Coords { x: coords.x, y: coords.y - 1 },
            Coords { x: coords.x + 1, y: coords.y },
            Coords { x: coords.x, y: coords.y + 1 },
            Coords { x: coords.x - 1, y: coords.y },
        ];

        for (i, neighbor) in neighbors.iter().enumerate() {
            if !region.contains(&neighbor) {
                border_sides[i].push(coords);
            }
        }
    }

    let mut side_count = 0;

    for (i, borders) in border_sides.iter_mut().enumerate() {
        if i % 2 == 0 {
            borders.sort_by_key(|&b| (b.y, b.x));
        } else {
            borders.sort_by_key(|&b| (b.x, b.y));
        }

        side_count += 1;
        let mut prev_coords = borders[0];

        for &coords in borders[1..].iter() {
            if i % 2 == 0 && (coords.y != prev_coords.y || coords.x.abs_diff(prev_coords.x) > 1) {
                side_count += 1;
            } else if i % 2 == 1 && (coords.x != prev_coords.x || coords.y.abs_diff(prev_coords.y) > 1) {
                side_count += 1;
            }

            prev_coords = coords;
        }
    }

    return side_count;
}

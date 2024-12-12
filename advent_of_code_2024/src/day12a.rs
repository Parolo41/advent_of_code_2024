use std::{collections::HashMap, fs};

#[derive(PartialEq, Clone, Copy, Debug)]
struct Coords { x: usize, y: usize }

impl Coords {
    fn next_to(&self, other: Coords) -> bool {
        (self.x == other.x || self.y == other.y)
        && (self.x.abs_diff(other.x) == 1 || self.y.abs_diff(other.y) == 1)
    }

    fn count_fences(&self, region: &Vec<Coords>) -> u32 {
        !region.contains(&Coords{ x: self.x, y: self.y - 1}) as u32
            + !region.contains(&Coords{ x: self.x + 1, y: self.y}) as u32
            + !region.contains(&Coords{ x: self.x, y: self.y + 1}) as u32
            + !region.contains(&Coords{ x: self.x - 1, y: self.y}) as u32
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

    for region in regions.iter() {
        let mut region_fences = 0;

        for coords in region {
            region_fences += coords.count_fences(region);
        }

        price_sum += region_fences * region.len() as u32;
    }

    println!("Total cost: {price_sum}");
}

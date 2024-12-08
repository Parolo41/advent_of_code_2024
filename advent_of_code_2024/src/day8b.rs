use std::fs;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, OutOfBoundsError>;

#[derive(Debug, Clone)]
struct OutOfBoundsError;

pub fn run() {
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    let file = fs::read_to_string("inputs/day8.txt").unwrap();

    let file_lines = file.lines();

    let mut y_bound = 0;
    let mut x_bound = 0;

    for (line_num, line) in file_lines.enumerate() {
        if line_num == 0 { x_bound = line.len() as i32 }
        y_bound += 1;

        for (byte_num, byte) in line.bytes().enumerate() {
            if byte != b'.' {
                let antenna_coords = antennas.entry(byte).or_insert(Vec::new());
                antenna_coords.push((byte_num as i32, line_num as i32));
            }
        }
    }
    
    for antenna_coords in antennas {
        for (i, coords) in antenna_coords.1.clone().iter().enumerate() {
            for target_coords in antenna_coords.1.clone()[i+1..].iter() {
                trace_coords(coords, target_coords, x_bound, y_bound, &mut antinodes);
            }
        }
    }

    println!("Number of Antinodes: {}", antinodes.len());
}

fn slingshot_coords((x1, y1): &(i32, i32), (x2, y2): &(i32, i32), x_bound: i32, y_bound: i32) -> Result<(i32, i32)> {
    let new_x = x2 + (x2 - x1);
    let new_y = y2 + (y2 - y1);

    if new_x < 0 || new_x >= x_bound
        || new_y < 0 || new_y >= y_bound {
            return Err(OutOfBoundsError);
        }

    Ok((new_x, new_y))
}

fn trace_coords(coord1: &(i32, i32), coord2: &(i32, i32), x_bound: i32, y_bound: i32, traced_coords: &mut Vec<(i32, i32)>) {
    push_if_unique(coord1, traced_coords);
    push_if_unique(coord2, traced_coords);

    let mut start_coord = *coord1;
    let mut target_coord = *coord2;

    loop {
        if let Ok(new_coord) = slingshot_coords(&start_coord, &target_coord, x_bound, y_bound) {
            push_if_unique(&new_coord, traced_coords);
            start_coord = target_coord;
            target_coord = new_coord;
        } else {
            break;
        }
    }

    let mut start_coord = *coord2;
    let mut target_coord = *coord1;

    loop {
        if let Ok(new_coord) = slingshot_coords(&start_coord, &target_coord, x_bound, y_bound) {
            push_if_unique(&new_coord, traced_coords);
            start_coord = target_coord;
            target_coord = new_coord;
        } else {
            break;
        }
    }
}

fn push_if_unique(coord: &(i32, i32), vec: &mut Vec<(i32, i32)>) {
    if !vec.contains(coord) {
        vec.push(*coord);
    }
}

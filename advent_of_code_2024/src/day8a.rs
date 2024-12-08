use std::fs;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, OutOfBoundsError>;

#[derive(Debug, Clone)]
struct OutOfBoundsError;

pub fn run() {
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes = Vec::new();

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
                if let Ok(new_coord) = slingshot_coords(coords, target_coords, x_bound, y_bound) {
                    if !antinodes.contains(&new_coord) {
                        antinodes.push(new_coord);
                    }
                }

                if let Ok(new_coord) = slingshot_coords(target_coords, coords, x_bound, y_bound) {
                    if !antinodes.contains(&new_coord) {
                        antinodes.push(new_coord);
                    }
                }
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

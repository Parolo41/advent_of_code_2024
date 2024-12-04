use std::fs;

pub fn run() {
    let mut xmas_count = 0;

    let mut lines = Vec::new();
    let mut x_coords = Vec::new();

    let file = fs::read_to_string("inputs/day4.txt").unwrap();

    for (line_num, line) in file.lines().enumerate() {
        let line_bytes = line.as_bytes();

        for (index, byte) in line_bytes.iter().enumerate() {
            if byte == &b'X' {
                x_coords.push([ line_num as i32, index as i32 ]);
            }
        }

        lines.push(line_bytes);
    }

    for c in x_coords {
        for x in -1..=1 {
            for y in -1..=1 {
                xmas_count += is_xmas(lines.clone(), c, [x, y]) as u32;
            }
        }
    }

    println!("XMAS found: {}", xmas_count);
}

fn is_xmas(lines: Vec<&[u8]>, c: [i32; 2], dir: [i32; 2]) -> bool {
    let wanted_string = [ b'X', b'M', b'A', b'S' ];
    let mut cursor = c.clone();

    for char in wanted_string {
        if cursor[0] < 0 || cursor[0] > (lines.len() - 1) as i32 { return false }
        if cursor[1] < 0 || cursor[1] > (lines[cursor[0] as usize].len() - 1) as i32 { return false }

        if lines[cursor[0] as usize][cursor[1] as usize] != char {
            return false;
        }

        cursor[0] += dir[0];
        cursor[1] += dir[1];
    }

    println!("XMAS found {c:?} {dir:?}");

    return true;
}

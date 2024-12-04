use std::fs;

pub fn run() {
    let mut x_mas_count = 0;

    let mut lines = Vec::new();
    let mut a_coords = Vec::new();

    let file = fs::read_to_string("inputs/day4.txt").unwrap();

    for (line_num, line) in file.lines().enumerate() {
        let line_bytes = line.as_bytes();

        for (index, byte) in line_bytes.iter().enumerate() {
            if byte == &b'A' {
                a_coords.push([ line_num, index ]);
            }
        }

        lines.push(line_bytes);
    }

    for c in a_coords {
        x_mas_count += is_x_mas(lines.clone(), c) as u32;
    }

    println!("X-MAS found: {}", x_mas_count);
}

fn is_x_mas(lines: Vec<&[u8]>, c: [usize; 2]) -> bool {
    if c[0] <= 0 || c[0] >= lines.len() - 1 || c[1] <= 0 || c[1] >= lines[0].len() - 1 {
        return false;
    }

    let mas1 = (lines[c[0] - 1][c[1] - 1] == b'M' && lines[c[0] + 1][c[1] + 1] == b'S') ||
        (lines[c[0] + 1][c[1] + 1] == b'M' && lines[c[0] - 1][c[1] - 1] == b'S');

    let mas2 = (lines[c[0] - 1][c[1] + 1] == b'M' && lines[c[0] + 1][c[1] - 1] == b'S') ||
        (lines[c[0] + 1][c[1] - 1] == b'M' && lines[c[0] - 1][c[1] + 1] == b'S');

    return mas1 && mas2;
}

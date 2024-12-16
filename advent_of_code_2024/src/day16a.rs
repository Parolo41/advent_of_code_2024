use core::panic;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day16.txt").unwrap();
    let mut lines = input.lines();

    let mut tiles = Vec::new();
    let mut height = 0;

    let mut start: (usize, usize) = (0, 0);

    while let Some(line) = lines.next() {
        let mut byte_line = Vec::new();

        for (i, byte) in line.as_bytes().iter().enumerate() {
            byte_line.push(*byte);

            if *byte == b'S' {
                start = (height, i);
            }
        }

        tiles.push(byte_line);
        height += 1;
    }

    let mut best_path = 0;

    let mut paths = vec![vec![start]];

    while let Some(mut path) = paths.pop() {
        if let Some(last) = path.last() {
            if tiles[last.0][last.1] == b'E' {
                let score = score_path(&path);

                if best_path == 0 || score < best_path {
                    best_path = score;
                }
            }
        }

        loop {
            let ways = get_ways(&path, &tiles);

            if ways.len() == 1 {
                path.push(ways[0]);
            } else {
                for way in ways {
                    let mut cloned_path = path.clone();
                    cloned_path.push(way);
                    paths.push(cloned_path);
                }
                break;
            }
        }
    }
}

fn get_ways(path: &Vec<(usize, usize)>, tiles: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut ways = Vec::new();

    let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    if let Some(cur_pos) = path.last() {
        for neighbor in neighbors {
            let neighbor_pos = (cur_pos.0 + neighbor.0 as usize, cur_pos.1 + neighbor.1 as usize);
            let neighbor_byte = tiles[neighbor_pos.0][neighbor_pos.1];
            if (neighbor_byte == b'.' || neighbor_byte == b'E') && !path.contains(&neighbor_pos) {
                ways.push((cur_pos.0 + neighbor.0 as usize, cur_pos.1 + neighbor.1 as usize));
            }
        }
    } else {
        panic!("Empty path in get_ways");
    }

    return ways;
}

fn score_path(path: &mut Vec<(usize, usize)>) -> i32 {
    let mut score = path.len() as i32 - 1;
    let mut prev_dir = (0, 1);
    let mut prev_pos = path.remove(0);

    for pos in path {
        let dir = get_dir(prev_pos, *pos);
    }

    return score;
}

fn get_dir(start: (usize, usize), end: (usize, usize)) -> (i32, i32) {
    ((end.0 as i32) - (start.0 as i32), (end.1 as i32) - (start.1 as i32))
}
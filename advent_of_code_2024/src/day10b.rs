use std::fs;

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct HeightMap {
    map: Vec<Vec<u32>>,
}

impl Direction {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn next(&self) -> Option<Direction> {
        match self {
            Direction::Up => Some(Direction::Right),
            Direction::Right => Some(Direction::Down),
            Direction::Down => Some(Direction::Left),
            Direction::Left => None,
        }
    }

    fn first() -> Direction {
        Direction::Up
    }
}

impl HeightMap {
    fn trail_head_score(&self, start_pos: (usize, usize)) -> u32 {
        if self.map[start_pos.0][start_pos.1] != 0 {
            return 0;
        }

        let mut score = 0;

        let mut height = 0;
        let mut pos = start_pos;
        let mut direction = Direction::first();
        let mut path = Vec::new();

        'score_loop: loop {
            if let Some(neightbor_height) = self.get_neighbor(pos, &direction) {
                if neightbor_height == height + 1 {
                    height = neightbor_height;
                    pos = direction.apply(pos);
                    path.push(direction.clone());
                    direction = Direction::first();

                    if height == 9 {
                        score += 1;
                    }

                    continue;
                }
            }

            if let Some(next_direction) = direction.next() {
                direction = next_direction;

                continue;
            }

            loop {
                if let Some(return_direction) = path.pop() {
                    if let Some(return_height) = self.get_neighbor(pos, &return_direction.reverse()) {
                        height = return_height;
                        pos = return_direction.reverse().apply(pos);

                        if let Some(continue_direction) = return_direction.next() {
                            direction = continue_direction;
                        } else {
                            continue;
                        }
                        
                        break;
                    } else {
                        panic!("Retracing path somehow lead to invalid position");
                    }
                } else {
                    break 'score_loop;
                }
            }
        }

        return score;
    }

    fn get_neighbor(&self, pos: (usize, usize), dir: &Direction) -> Option<u32> {
        if (*dir == Direction::Up && pos.0 == 0)
            || (*dir == Direction::Right && pos.1 >= self.map[0].len() - 1)
            || (*dir == Direction::Down && pos.0 >= self.map.len() - 1)
            || (*dir == Direction::Left && pos.1 == 0) {
            return None;
        }

        let neighbor_pos = dir.apply(pos);
        
        Some(self.map[neighbor_pos.0][neighbor_pos.1])
    }
}

pub fn run() {
    let mut map = Vec::new();

    for line in fs::read_to_string("inputs/day10.txt").unwrap().lines() {
        map.push(line.chars().map(|c| c.to_digit(10).expect("File should only contain numeric chars")).collect::<Vec<_>>());
    }

    let height_map = HeightMap {
        map,
    };

    let mut score_sum = 0;

    for (y, row) in height_map.map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                score_sum += height_map.trail_head_score((y, x));
            }
        }
    }

    println!("Sum of scores: {score_sum}");
}

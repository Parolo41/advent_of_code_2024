use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn all() -> Vec<Dir> {
        vec![
            Dir::Up,
            Dir::Right,
            Dir::Down,
            Dir::Left,
        ]
    }
}

#[derive(Clone)]
struct Step {
    x: i32,
    y: i32,
    dir: Dir,
    score: i32,
}

impl Step {
    fn go_to(&self, dir: Dir) -> Step {
        let mut x = self.x;
        let mut y = self.y;
        let mut score = self.score + 1;

        match dir {
            Dir::Up => y -= 1,
            Dir::Right => x += 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
        }

        if dir != self.dir {
            score += 1000;
        }

        Step {
            x,
            y,
            score,
            dir,
        }
    }
}

#[derive(Clone)]
struct PathRegistry {
    steps: Vec<Step>,
}

impl PathRegistry {
    fn is_new_or_better(&mut self, new_step: &Step) -> bool {
        let temp_steps = self.steps.clone();

        for (i, old_step) in temp_steps.into_iter().enumerate() {
            if old_step.x == new_step.x && old_step.y == new_step.y {
                if new_step.score < old_step.score {
                    self.steps.remove(i);
                    self.steps.push(new_step.clone());
                    return true;
                } else {
                    return false;
                }
            }
        }

        self.steps.push(new_step.clone());
        return true;
    }
}

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

    let mut best_score = 0;

    let first_step = Step {
        x: start.1 as i32,
        y: start.0 as i32,
        dir: Dir::Right,
        score: 0,
    };

    let mut path_registry = PathRegistry {
        steps: Vec::new(),
    };
    path_registry.is_new_or_better(&first_step);
    let mut paths = vec![vec![first_step]];

    while let Some(path) = paths.pop() {
        let last_step = path.last().unwrap();

        if is_end(&last_step, &tiles) && (best_score == 0 || best_score > last_step.score) {
            best_score = last_step.score;
            println!("Path finished");
            continue;
        }

        for dir in Dir::all() {
            let next_step = last_step.go_to(dir);

            if is_valid(&next_step, &tiles) && !contains(&next_step, &path) {
                if path_registry.is_new_or_better(&next_step) {
                    let mut cloned_path = path.clone();
                    cloned_path.push(next_step);
                    paths.push(cloned_path);
                } else {
                    let next_next_step = next_step.go_to(dir);

                    if is_valid(&next_next_step, &tiles) && path_registry.is_new_or_better(&next_next_step) {
                        let mut cloned_path = path.clone();
                        cloned_path.push(next_step);
                        cloned_path.push(next_next_step);
                        paths.push(cloned_path);
                    }
                }
            }
        }
    }

    println!("Best score: {best_score}");
}

fn is_valid(step: &Step, tiles: &Vec<Vec<u8>>) -> bool {
    let tile = tiles[step.y as usize][step.x as usize];

    tile == b'.' || tile == b'E'
}

fn is_end(step: &Step, tiles: &Vec<Vec<u8>>) -> bool {
    let tile = tiles[step.y as usize][step.x as usize];

    tile == b'E'
}

fn contains(step: &Step, path: &Vec<Step>) -> bool {
    for path_step in path {
        if step.x == path_step.x && step.y == path_step.y {
            return true;
        }
    }

    return false;
}
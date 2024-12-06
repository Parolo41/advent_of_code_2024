use std::fs;

enum Dir {
    Up,
    Right,
    Down,
    Left
}

struct Map {
    bounds: (usize, usize),
    obstacles: Vec<(usize, usize)>,
    player: (usize, usize),
    player_dir: Dir,
    visited: Vec<(usize, usize)>,
}

impl Map {
    fn turn_right(&mut self) {
        match self.player_dir {
            Dir::Up => self.player_dir = Dir::Right,
            Dir::Right => self.player_dir = Dir::Down,
            Dir::Down => self.player_dir = Dir::Left,
            Dir::Left => self.player_dir = Dir::Up,
        }
    }

    fn move_forward(&mut self) -> bool {
        while self.is_next_blocked() {
            self.turn_right();
        }

        if self.is_next_oob() {
            return false;
        } else {
            match self.player_dir {
                Dir::Up => self.player = (self.player.0, self.player.1 - 1),
                Dir::Right => self.player = (self.player.0 + 1, self.player.1),
                Dir::Down => self.player = (self.player.0, self.player.1 + 1),
                Dir::Left => self.player = (self.player.0 - 1, self.player.1),
            }

            return true;
        }
    }

    fn is_next_oob(&self) -> bool {
        match self.player_dir {
            Dir::Up => self.player.1 == 0,
            Dir::Right => self.player.0 == self.bounds.0,
            Dir::Down => self.player.1 == self.bounds.1,
            Dir::Left => self.player.0 == 0,
        }
    }

    fn is_next_blocked(&self) -> bool {
        match self.player_dir {
            Dir::Up => self.obstacles.contains(&(self.player.0, self.player.1 - 1)),
            Dir::Right => self.obstacles.contains(&(self.player.0 + 1, self.player.1)),
            Dir::Down => self.obstacles.contains(&(self.player.0, self.player.1 + 1)),
            Dir::Left => self.obstacles.contains(&(self.player.0 - 1, self.player.1)),
        }
    }

    fn is_new_position(&mut self) -> bool {
        if self.visited.contains(&self.player) {
            return false;
        } else {
            self.visited.push(self.player.clone());

            return true;
        }
    }
}

pub fn run() {
    let file_lines = fs::read_to_string("inputs/day6.txt").unwrap();

    let mut x_bound = 0;
    let mut y_bound = 0;
    
    let mut obstacles = Vec::new();
    let mut player = (0, 0);
    let mut visited = Vec::new();

    for (line_num, line) in file_lines.lines().enumerate() {
        if line_num == 0 { x_bound = line.len(); }
        y_bound += 1;

        for (index, byte) in line.as_bytes().iter().enumerate() {
            if byte == &b'#' {
                obstacles.push((index, line_num));
            } else if byte == &b'^' {
                player = (index, line_num);
                visited.push((index, line_num));
            }
        }
    }

    let mut map = Map {
        bounds: (x_bound - 1, y_bound - 1),
        obstacles,
        player,
        player_dir: Dir::Up,
        visited,
    };

    let mut visited_count = 1;

    while map.move_forward() {
        visited_count += map.is_new_position() as i32;
    }

    println!("Positions visited: {visited_count}");
}

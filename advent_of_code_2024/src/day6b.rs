use std::fs;
use std::cmp;

#[derive(Copy, Clone, PartialEq)]
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
    turns_made: Vec<((usize, usize), Dir)>,
    obstructions: Vec<(usize, usize)>,
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

    fn move_to(&mut self, dest: (usize, usize)) {
        self.player = dest;
    }

    fn move_forward(&mut self) -> bool {
        if self.is_next_oob() {
            return false;
        } else {
            while self.is_blocked(self.forward_position()) {
                self.turn_right();
                self.register_turn();
                self.check_obstruction_spot();
            }

            self.move_to(self.forward_position());

            if !self.is_left_oob() && self.is_blocked(self.left_position()) {
                self.register_turn();
            }

            if !self.visited.contains(&self.player) {
                self.visited.push(self.player.clone());
            }

            self.check_obstruction_spot();

            return true;
        }
    }

    fn register_turn(&mut self) {
        self.turns_made.push((self.player, self.player_dir));
    }

    fn check_obstruction_spot(&mut self) {
        if !self.is_next_oob() 
            && self.has_doubled_back() 
            && !self.is_visited(self.forward_position())
            && !self.obstructions.contains(&self.forward_position()) {
            self.obstructions.push(self.forward_position());

            println!("New obstruction at {:?}: {:?}", self.forward_position(), self.player);
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

    fn is_left_oob(&self) -> bool {
        match self.player_dir {
            Dir::Up => self.player.0 == 0,
            Dir::Right => self.player.1 == 0,
            Dir::Down => self.player.0 == self.bounds.0,
            Dir::Left => self.player.1 == self.bounds.1,
        }
    }

    fn is_blocked(&self, dest: (usize, usize)) -> bool {
        self.obstacles.contains(&dest)
    }

    fn is_visited(&self, dest: (usize, usize)) -> bool {
        self.visited.contains(&dest)
    }

    fn has_doubled_back(&self) -> bool {
        let target_dir = match self.player_dir {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        };

        for turn in self.turns_made.clone() {
            if turn.1 == target_dir {
                match self.player_dir {
                    Dir::Up => {
                        if turn.0.1 == self.player.1
                            && turn.0.0 > self.player.0
                            && self.has_path_to(turn.0) {
                            return true;
                        }
                    },
                    Dir::Right => {
                        if turn.0.0 == self.player.0
                            && turn.0.1 > self.player.1
                            && self.has_path_to(turn.0) {
                            return true;
                        }
                    },
                    Dir::Down => {
                        if turn.0.1 == self.player.1
                            && turn.0.0 < self.player.0
                            && self.has_path_to(turn.0) {
                            return true;
                        }
                    },
                    Dir::Left => {
                        if turn.0.0 == self.player.0
                            && turn.0.1 < self.player.1
                            && self.has_path_to(turn.0) {
                            return true;
                        }
                    },
                }
            }
        }

        return false;
    }

    fn has_path_to(&self, dest: (usize, usize)) -> bool {
        if self.player.0 == dest.0 {
            let start = cmp::min(self.player.1, dest.1);
            let end = cmp::max(self.player.1, dest.1);

            for i in start..end {
                if self.is_blocked((dest.0, i)) {
                    return false;
                }
            }
        } else if self.player.1 == dest.1 {
            let start = cmp::min(self.player.0, dest.0);
            let end = cmp::max(self.player.0, dest.0);

            for i in start..end {
                if self.is_blocked((dest.1, i)) {
                    return false;
                }
            }
        } else {
            return false;
        }

        return true;
    }

    fn forward_position(&self) -> (usize, usize) {
        match self.player_dir {
            Dir::Up => (self.player.0, self.player.1 - 1),
            Dir::Right => (self.player.0 + 1, self.player.1),
            Dir::Down => (self.player.0, self.player.1 + 1),
            Dir::Left => (self.player.0 - 1, self.player.1),
        }
    }

    fn left_position(&self) -> (usize, usize) {
        match self.player_dir {
            Dir::Up => (self.player.0 - 1, self.player.1),
            Dir::Right => (self.player.0, self.player.1 - 1),
            Dir::Down => (self.player.0 + 1, self.player.1),
            Dir::Left => (self.player.0, self.player.1 + 1),
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
        turns_made: Vec::new(),
        obstructions: Vec::new(),
    };

    loop {
        if !map.move_forward() { break; }
    }

    println!("Number of possible obstructions: {}", map.obstructions.len());
    println!("Number of visited tiles: {}", map.visited.len());
}

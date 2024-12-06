use std::fs;

#[derive(Copy, Clone, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left
}

enum MoveResult {
    Ok,
    Ended,
    Looping,
}

#[derive(Clone)]
struct Map {
    bounds: (usize, usize),
    obstacles: Vec<(usize, usize)>,
    player: (usize, usize),
    player_dir: Dir,
    turns_made: Vec<((usize, usize), Dir)>,
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

    fn move_to(&mut self, dest: (usize, usize)) {
        self.player = dest;
    }

    fn move_forward(&mut self) -> MoveResult {
        if self.is_next_oob() {
            return MoveResult::Ended;
        } else {
            while self.is_blocked(self.forward_position()) {
                self.turn_right();

                if self.turns_made.contains(&(self.player.clone(), self.player_dir)) {
                    return MoveResult::Looping;
                }

                self.register_turn();
            }

            self.move_to(self.forward_position());

            if !self.visited.contains(&self.player) {
                self.visited.push(self.player.clone());
            }

            return MoveResult::Ok;
        }
    }

    fn register_turn(&mut self) {
        self.turns_made.push((self.player, self.player_dir));
    }

    fn is_next_oob(&self) -> bool {
        match self.player_dir {
            Dir::Up => self.player.1 == 0,
            Dir::Right => self.player.0 == self.bounds.0,
            Dir::Down => self.player.1 == self.bounds.1,
            Dir::Left => self.player.0 == 0,
        }
    }

    fn is_blocked(&self, dest: (usize, usize)) -> bool {
        self.obstacles.contains(&dest)
    }

    fn forward_position(&self) -> (usize, usize) {
        match self.player_dir {
            Dir::Up => (self.player.0, self.player.1 - 1),
            Dir::Right => (self.player.0 + 1, self.player.1),
            Dir::Down => (self.player.0, self.player.1 + 1),
            Dir::Left => (self.player.0 - 1, self.player.1),
        }
    }

    fn does_loop (&mut self) -> bool {
        loop {
            let move_result = self.move_forward();

            match move_result {
                MoveResult::Ended => return false,
                MoveResult::Looping => return true,
                _ => (),
            }
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
        obstacles: obstacles.clone(),
        player: player.clone(),
        player_dir: Dir::Up,
        turns_made: Vec::new(),
        visited: Vec::new(),
    };

    loop {
        if let MoveResult::Ended = map.move_forward() {
            break;
        }
    }

    let mut looping_instances = 0;
    let mut count = 0;

    for visited in map.visited.clone() {
        let mut copied_map = Map {
            bounds: (x_bound - 1, y_bound - 1),
            obstacles: obstacles.clone(),
            player: player.clone(),
            player_dir: Dir::Up,
            turns_made: Vec::new(),
            visited: Vec::new(),
        };
    
        if copied_map.obstacles.contains(&visited) 
            || (visited.0 == player.0 && visited.1 == player.1) {
            continue;
        }

        copied_map.obstacles.push(visited);
    
        looping_instances += copied_map.does_loop() as i32;
        count += 1;
        println!("{} - {:?}: Possible obstacle count: {}", count, visited, looping_instances);
    }

    println!("Possible obstacle count: {looping_instances}");
}

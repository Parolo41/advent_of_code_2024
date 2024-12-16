use std::fs;

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Right => (pos.0, pos.1 + 1),
            Dir::Down => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0, pos.1 - 1),
        }
    }

    fn from_byte(byte: u8) -> Dir {
        match byte {
            b'^' => Dir::Up,
            b'>' => Dir::Right,
            b'v' => Dir::Down,
            b'<' => Dir::Left,
            _ => panic!("Invalid byte"),
        }
    }
}

struct Map {
    tiles: Vec<Vec<u8>>,
    player: (usize, usize),
}

impl Map {
    fn do_move(&mut self, dir: Dir) {
        let dest = self.get(dir.apply(self.player));

        match dest {
            b'.' => self.player = dir.apply(self.player),
            b'#' => (),
            b'O' => if self.push(dir.apply(self.player), dir) {
                self.set(dir.apply(self.player), b'.');
                self.player = dir.apply(self.player);
            },
            _ => panic!("Invalid map tile"),
        }
    }

    fn push(&mut self, pos: (usize, usize), dir: Dir) -> bool {
        let dest = self.get(dir.apply(pos));

        match dest {
            b'.' => {
                self.set(dir.apply(pos), b'O');
                true
            },
            b'#' => false,
            b'O' => self.push(dir.apply(pos), dir),
            _ => panic!("Invalid map tile"),
        }
    }

    fn get(&self, pos: (usize, usize)) -> u8 {
        self.tiles[pos.0][pos.1]
    }

    fn set(&mut self, pos: (usize, usize), byte: u8) {
        self.tiles[pos.0][pos.1] = byte;
    }
}

pub fn run() {
    let input = fs::read_to_string("inputs/day15.txt").unwrap();
    let mut lines = input.lines();
    let mut height = 0;

    let mut tiles = Vec::new();

    let mut player = (0, 0);

    while let Some(line) = lines.next() {
        if line.len() == 0 { break; }

        let mut byte_line = Vec::new();

        for (i, byte) in line.as_bytes().iter().enumerate() {
            if *byte == b'@' {
                player = (height, i);
                byte_line.push(b'.');
            } else {
                byte_line.push(*byte);
            }
        }

        height += 1;

        tiles.push(byte_line);
    }

    let mut map = Map {
        tiles,
        player,
    };

    while let Some(line) = lines.next() {
        for byte in line.as_bytes() {
            map.do_move(Dir::from_byte(*byte));
        }
    }

    let mut coord_sum = 0;

    for (y, line) in map.tiles.iter().enumerate() {
        for (x, byte) in line.iter().enumerate() {
            if *byte == b'O' {
                coord_sum += (y * 100) + x;
            }
            print!("{}", *byte as char);
        }
        println!();
    }

    println!("Coordinates sum: {coord_sum}");
}
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
        if self.push(dir.apply(self.player), dir) {
            self.player = dir.apply(self.player);
        }
    }

    fn push(&mut self, pos: (usize, usize), dir: Dir) -> bool {
        let here = self.get(pos);

        match here {
            b'.' => return true,
            b'#' => return false,
            b'[' | b']' => {
                match dir {
                    Dir::Up | Dir::Down => {
                        let (left, right, other_pos): ((usize, usize), (usize, usize), (usize, usize));
                        if here == b'[' {
                            left = dir.apply(pos);
                            right = (left.0, left.1 + 1);
                            other_pos = (pos.0, pos.1 + 1);
                        } else {
                            right = dir.apply(pos);
                            left = (right.0, right.1 - 1);
                            other_pos = (pos.0, pos.1 - 1);
                        }

                        if self.pushable(left, dir) && self.pushable(right, dir) {
                            self.push(left, dir);
                            self.push(right, dir);
                            self.set(pos, b'.');
                            self.set(other_pos, b'.');
                            self.set(left, b'[');
                            self.set(right, b']');
                            return true;
                        } else {
                            return false;
                        }
                    },
                    Dir::Right | Dir::Left => { 
                        if self.push(dir.apply(pos), dir) {
                            self.set(pos, b'.');
                            self.set(dir.apply(pos), here);
                            return true;
                        } else {
                            return false;
                        }
                    },
                }
            },
            _ => panic!("Invalid byte"),
        }
    }

    fn pushable(&self, pos: (usize, usize), dir: Dir) -> bool {
        let here = self.get(pos);

        match here {
            b'.' => return true,
            b'#' => return false,
            b'[' | b']' => {
                match dir {
                    Dir::Up | Dir::Down => {
                        let (left, right): ((usize, usize), (usize, usize));
                        if here == b'[' {
                            left = dir.apply(pos);
                            right = (left.0, left.1 + 1);
                        } else {
                            right = dir.apply(pos);
                            left = (right.0, right.1 - 1);
                        }

                        return self.pushable(left, dir) && self.pushable(right, dir);
                    },
                    Dir::Right | Dir::Left => { 
                        return self.pushable(dir.apply(pos), dir);
                    },
                }
            },
            _ => panic!("Invalid byte"),
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
                player = (height, i * 2);
                byte_line.push(b'.');
                byte_line.push(b'.');
            } else if *byte == b'O' {
                byte_line.push(b'[');
                byte_line.push(b']');
            } else {
                byte_line.push(*byte);
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
            // print_map(&map);
        }
    }

    let mut coord_sum = 0;

    for (y, line) in map.tiles.iter().enumerate() {
        for (x, byte) in line.iter().enumerate() {
            if *byte == b'[' {
                coord_sum += (y * 100) + x;
            }
        }
    }

    print_map(&map);

    println!("Coordinates sum: {coord_sum}");
}

fn print_map(map: &Map) {
    for (y, line) in map.tiles.iter().enumerate() {
        for (x, byte) in line.iter().enumerate() {
            if (y, x) == map.player {
                print!("@");
            } else {
                print!("{}", *byte as char);
            }
        }
        println!();
    }
}
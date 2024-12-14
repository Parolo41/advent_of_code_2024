use std::fs;

use regex::Regex;

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;

struct Robot {
    x: i32,
    y: i32,
    x_v: i32,
    y_v: i32,
}

impl Robot {
    fn do_move(&mut self) {
        self.x += self.x_v;
        self.y += self.y_v;

        if self.x >= WIDTH {
            self.x -= WIDTH;
        }

        if self.y >= HEIGHT {
            self.y -= HEIGHT;
        }

        if self.x < 0 {
            self.x += WIDTH;
        }

        if self.y < 0 {
            self.y += HEIGHT;
        }
    }
}

pub fn run() {
    let mut robots = Vec::new();

    let input = fs::read_to_string("inputs/day14.txt").unwrap();
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    for robot_vals in robot_regex.captures_iter(&input[..]) {
        robots.push(Robot {
            x: robot_vals[1].parse().unwrap(),
            y: robot_vals[2].parse().unwrap(),
            x_v: robot_vals[3].parse().unwrap(),
            y_v: robot_vals[4].parse().unwrap(),
        });
    }

    let mut move_count = 0;

    loop {
        let mut grid = [[false; HEIGHT as usize]; WIDTH as usize];

        for robot in robots.iter_mut() {
            robot.do_move();

            grid[robot.x as usize][robot.y as usize] = true;
        }

        let mut counter = 0;

        'checker_loop: for column in grid {
            for pos in column {
                if pos {
                    counter += 1;
                    if counter >= 8 {
                        break 'checker_loop;
                    }
                } else {
                    counter = 0;
                }
            }
        }

        move_count += 1;

        if counter < 8 {
            continue;
        }

        for column in grid {
            for pos in column {
                if pos {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!("Moves: {move_count}");
        let mut buffer= String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
    }
}
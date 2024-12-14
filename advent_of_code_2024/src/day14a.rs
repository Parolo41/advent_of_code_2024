use std::fs;

use regex::Regex;

static HEIGHT: i32 = 103;
static WIDTH: i32 = 101;

static MOVES: i32 = 100;

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

    for _ in 0..MOVES {
        for robot in robots.iter_mut() {
            robot.do_move();
        }
    }

    let (mut section1, mut section2, mut section3, mut section4) = (0, 0, 0, 0);

    let x_mid = WIDTH / 2;
    let y_mid = HEIGHT / 2;

    for robot in robots {
        if robot.x < x_mid && robot.y < y_mid {
            section1 += 1;
        } else if robot.x > x_mid && robot.y < y_mid {
            section2 += 1;
        } else if robot.x < x_mid && robot.y > y_mid {
            section3 += 1;
        } else if robot.x > x_mid && robot.y > y_mid {
            section4 += 1;
        }
    }

    println!("Safety score: {}", section1 * section2 * section3 * section4);
}
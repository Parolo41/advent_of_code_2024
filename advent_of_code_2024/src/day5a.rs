use std::fs;

pub fn run() {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut updates_reached = false;

    let mut page_num_sum = 0;

    for line in fs::read_to_string("inputs/day5.txt").unwrap().lines() {
        if line.len() == 0 {
            updates_reached = true;
            continue;
        }

        if updates_reached {
            updates.push(line.split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>());
        } else {
            rules.push(line.split("|").map(|s| s.parse().unwrap()).collect::<Vec<i32>>());
        }
    }

    'update_loop: for update in updates {
        for rule in &rules[..] {
            if !update.contains(&rule[0])  || !update.contains(&rule[1]) {
                continue;
            }

            if update.iter().position(|&v| v == rule[0]).unwrap() > update.iter().position(|&v| v == rule[1]).unwrap() {
                continue 'update_loop;
            }
        }

        page_num_sum += update[(update.len() - 1) / 2];
    }

    println!("Sum of middle page numbers: {page_num_sum}");
}

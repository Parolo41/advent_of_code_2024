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

    for update in updates {
        let mut occurrence_counter = vec![0; update.len()];
        let mut update_flawed = false;

        for rule in &rules {
            if !update.contains(&rule[0])  || !update.contains(&rule[1]) {
                continue;
            }

            let first_val_pos = update.iter().position(|&v| v == rule[0]).unwrap();
            let second_val_pos = update.iter().position(|&v| v == rule[1]).unwrap();

            occurrence_counter[first_val_pos] += 1;

            if first_val_pos > second_val_pos {
                update_flawed = true;
            }
        }

        if update_flawed {
            for (index, page_occurrences) in occurrence_counter.into_iter().enumerate() {
                if page_occurrences == (update.len() - 1) / 2 {
                    page_num_sum += update[index];
                }
            }
        }
    }

    println!("Sum of middle page numbers: {page_num_sum}");
}

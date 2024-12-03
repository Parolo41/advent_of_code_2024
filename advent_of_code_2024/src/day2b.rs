use std::fs;

pub fn run() {
    let mut safe_count = 0;

    for line in fs::read_to_string("inputs/day2.txt").unwrap().lines() {
        let values = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        safe_count += is_safe(values) as i32
    }

    println!("Safe report count: {safe_count}");
}

fn is_safe(values: Vec<i32>) -> bool {
    let mut dirty_indices = Vec::new();
    let mut increases = 0;
    let mut decreases = 0;

    let mut old_direction = (values[0] < values[1]) as i8 - (values[0] > values[1]) as i8;

    // Search for indices with direction changes and mark them as dirty
    for i in 0 .. values.len() - 1 {
        let increasing = values[i] < values[i + 1];
        let decreasing = values[i] > values[i + 1];

        increases += increasing as i8;
        decreases += decreasing as i8;

        let new_direction = increasing as i8 - decreasing as i8;

        if new_direction == 0 {
            dirty_indices.push(i);
        } else if new_direction != old_direction {
            dirty_indices.push(i);

            old_direction = new_direction;
        }
    }

    // If the second elements has a direction change, check the first element too
    if dirty_indices.contains(&1) {
        dirty_indices.push(0);
    }

    // If the second to last elements has a direction change, check the last element too
    if dirty_indices.contains(&(values.len() - 2)) {
        dirty_indices.push(values.len() - 1);
    }

    // Check if beginning or end have a spike, if yes mark them as dirty
    if values[0].abs_diff(values[1]) > 3 { dirty_indices.push(0); }
    if values[values.len() - 2].abs_diff(values[values.len() - 1]) > 3 { dirty_indices.push(values.len() - 1)}

    if increases == 0 || decreases == 0 {
        // Spikes in the middle of reports without direction changes can't be fixed
        for i in 1 .. values.len() - 2 {
            if values[i].abs_diff(values[i + 1]) > 3 { return false; }
        }

        // If no direction changes or spikes, it's safe
        if dirty_indices.len() == 0 { return true; }
    }
    
    // If the report goes in neither direction enough, it can't be fixed
    if increases <  (values.len() - 2) as i8 && decreases < (values.len() - 2) as i8 { return false; }

    // Remove dirty indices one at a time and check again
    for index in dirty_indices {
        let mut cleaned_values = values.to_vec();
        cleaned_values.remove(index);

        if is_cleaned_safe(cleaned_values) { return true; }
    }

    // If it wasn't fixed, it's unsafe
    return false;
}

fn is_cleaned_safe(values: Vec<i32>) -> bool {
    let increasing = values[0] < values[1];

    for i in 1..values.len() {
        if increasing && values[i-1] >= values[i] { return false; }
        if !increasing && values[i-1] <= values[i] { return false; }
        if values[i-1].abs_diff(values[i]) > 3 { return false; }
    }

    return true;
}

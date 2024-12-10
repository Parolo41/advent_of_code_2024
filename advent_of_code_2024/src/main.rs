pub mod day1a;
pub mod day1b;
pub mod day2a;
pub mod day2b;
pub mod day3a;
pub mod day3b;
pub mod day4a;
pub mod day4b;
pub mod day5a;
pub mod day5b;
pub mod day6a;
pub mod day6b;
pub mod day7a;
pub mod day7b;
pub mod day8a;
pub mod day8b;
pub mod day9a;
pub mod day9b;
pub mod day10a;
pub mod day10b;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let module = "day10b";

    match module {
        "day1a" => day1a::run(),
        "day1b" => day1b::run(),
        "day2a" => day2a::run(),
        "day2b" => day2b::run(),
        "day3a" => day3a::run(),
        "day3b" => day3b::run(),
        "day4a" => day4a::run(),
        "day4b" => day4b::run(),
        "day5a" => day5a::run(),
        "day5b" => day5b::run(),
        "day6a" => day6a::run(),
        "day6b" => day6b::run(),
        "day7a" => day7a::run(),
        "day7b" => day7b::run(),
        "day8a" => day8a::run(),
        "day8b" => day8b::run(),
        "day9a" => day9a::run(),
        "day9b" => day9b::run(),
        "day10a" => day10a::run(),
        "day10b" => day10b::run(),
        _=> println!("Invalid module"),
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

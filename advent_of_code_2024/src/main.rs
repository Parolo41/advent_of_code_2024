pub mod day1a;
pub mod day1b;
pub mod day2a;
pub mod day2b;
pub mod day3a;
pub mod day3b;

fn main() {
    let module = "day3b";

    match module {
        "day1a" => day1a::run(),
        "day1b" => day1b::run(),
        "day2a" => day2a::run(),
        "day2b" => day2b::run(),
        "day3a" => day3a::run(),
        "day3b" => day3b::run(),
        _=> println!("Invalid module"),
    }
}

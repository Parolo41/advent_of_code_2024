pub mod day1a;
pub mod day1b;
pub mod day2a;
pub mod day2b;

fn main() {
    let module = "day2b";

    match module {
        "day1a" => day1a::run(),
        "day1b" => day1b::run(),
        "day2a" => day2a::run(),
        "day2b" => day2b::run(),
        _=> println!("Invalid module"),
    }
}

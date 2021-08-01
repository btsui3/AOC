use rust::{day1::{day1_part1, day1_part2}, day2::{self, day2}};
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
    .map(|s|s.as_str())
    .unwrap_or("None");

    let result = match problem {
        "day1" => day1_part1(),
        "day1-part2" => day1_part2(),
        "day2" => day2(),
        _ => "Haven't done it yet".to_string()
    };
    println!("{}", result); 
}

use rust::day1::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
    .map(|s|s.as_str())
    .unwrap_or("None");

    let result = match problem {
        "day1" => sum_of_max_calories(),
        _ => "No problem found".to_string(),
    };
    println!("{}", result); 
}

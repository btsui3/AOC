use rust::day1::*;
use rust::day2::*;
use rust::day3::*;
use rust::day3_part2::get_item_priority_sum_part2;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
    .map(|s|s.as_str())
    .unwrap_or("None");

    let result = match problem {
        "day1" => sum_of_max_calories(),
        "day1_part2" => get_top_3_sum(),
        "day2" => final_score(),
        "day3" => get_item_priority_sum(),
        "day3_part2" => get_item_priority_sum_part2(),
        _ => "No problem found".to_string(),
    };
    println!("{}", result); 
}

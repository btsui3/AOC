use rust::day1::day1;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
    .map(|s|s.as_str())
    .unwrap_or("None");

    let result = match problem {
        "day1" => day1(),
        _ => "Haven't done it yet".to_string()
    };
    println!("{}", result); 
}

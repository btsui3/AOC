use std::fs;
use itertools::Itertools;

pub fn day1_part1() -> String {
    find_expense_product(2)
} 

pub fn day1_part2() -> String {
    find_expense_product(3)
} 

pub fn find_expense_product(n: usize) -> String {
    let values = read_data();
    let result :Option<usize> = values.iter().combinations(n)
    .find(|v| v.iter().copied().sum::<usize>() == 2020)
    .map(|v| v.into_iter().product::<usize>());
      match result {
        Some(v) => v.to_string(),
        None => "No solution".to_string()
    }
}

fn read_data() -> Vec<usize>{
    let values = fs::read_to_string("./src/inputs/input_day01.txt").expect("Could not read file");
    values
    .split('\n')
    .filter_map(|s|s.parse::<usize>().ok())
    .collect()
}

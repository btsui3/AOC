use crate::utility::read_data;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn get_item_priority_sum_part2() -> String {
    let input = read_data("./src/inputs/day3_input.txt");
    let mut item_priority_list : Vec<String> = Vec::new();  

    for line in input.lines() {
      item_priority_list.push(line.to_string());
  }

  return part2_sum(item_priority_list);
}

pub fn part2_sum(item_priority_list : Vec<String>) -> String {
    let item_priority_map = get_item_priority_map();
    let mut result = 0;
     for chunk in item_priority_list.chunks(3) {
        if chunk.len() == 3 {
            let first_string = &chunk[0];
            let second_string = &chunk[1];
            let third_string = &chunk[2];
            
            println!("{} {} {}", first_string, second_string, third_string);
            result += calculate_sum(&item_priority_map, first_string, second_string, third_string);
        }
    }

  return result.to_string();

}

pub fn get_item_priority_map() -> HashMap<char, i32> {
  let mut item_priority_map = HashMap::new();
  let mut priority = 1;
  for item in 'a'..='z' {
    item_priority_map.insert(item, priority);
    priority += 1;
  }

  let mut priority = 27;
  for item in 'A'..='Z' {
    item_priority_map.insert(item, priority);
    priority += 1;
  }

  return item_priority_map;
}

pub fn calculate_sum(item_priority_map: &HashMap<char, i32>, first_string: &str, second_string: &str, third_string: &str) -> i32 {
    let mut sum = 0;
    let mut hash_set = HashSet::new();

    for char in first_string.chars() {
        if second_string.contains(char) && third_string.contains(char) {
            if !hash_set.contains(&char) {
                hash_set.insert(char);
                sum += item_priority_map.get(&char).unwrap();
                println!("{:?} {:?}", char, sum);
            } else {
                continue;
            }
        }
    }

    sum
}

use crate::utility::read_data;
use std::collections::HashMap;

pub fn get_item_priority_sum() -> String {
    let input = read_data("./src/inputs/day3_input.txt");
    let mut item_priority_list : Vec<String> = Vec::new();  

    for line in input.lines() {
      item_priority_list.push(line.to_string());
  }

  return part1_sum(item_priority_list);
}

pub fn part1_sum(item_priority_list : Vec<String>) -> String {
  let item_priority_map = get_item_priority_map();
  let mut result = 0;

  for item in item_priority_list.iter() {
      let first_half_string = item.split_at(item.len() / 2).0;
      let second_half_string = item.split_at(item.len() / 2).1;

      println!("{:?} {:?} {:?}", item, first_half_string, second_half_string);

      result += calculate_sum(&item_priority_map, &first_half_string, &second_half_string);
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

pub fn calculate_sum(item_priority_map : &HashMap<char, i32>, first_half_string : &str, second_half_string : &str) -> i32 {
  let mut sum = 0;
  let mut hash_map = HashMap::new();
  for char in first_half_string.chars() {
    for char2 in second_half_string.chars() {
      if char == char2 {
        if !hash_map.contains_key(&char) {
          hash_map.insert(char, 1);
          sum += item_priority_map.get(&char).unwrap();
          println!("{:?} {:?}", char, sum);
        } else {
          continue;
        }
      }
    }
  }

  return sum;
}

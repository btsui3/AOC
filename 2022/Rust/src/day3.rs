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
  // Create hash map of item and priority
  //Lowercase item types a through z have priorities 1 through 26.
  //Uppercase item types A through Z have priorities 27 through 52.

  let item_priority_map = get_item_priority_map();

  // Divide list in half
  let mut first_half = Vec::new();
  let mut second_half = Vec::new();

  for (index, item) in item_priority_list.iter().enumerate() {
    if index < item_priority_list.len() / 2 {
      first_half.push(item);
    } else {
      second_half.push(item);
    }
  }

  // Find common character in each list
  let mut common_chars = Vec::new();
  for item in first_half {
    for item2 in &second_half {
      for (index, char) in item.chars().enumerate() {
        if char == item2.chars().nth(index).unwrap() {
          common_chars.push(char);
        }
      }
    }
  }

  // Find sum of common characters
  let mut sum = 0;

  for char in common_chars {
    sum += item_priority_map.get(&char).unwrap();
  }

  



  return sum.to_string();

}

pub fn get_item_priority_map() -> HashMap<char, i32> {
  let mut item_priority_map = HashMap::new();
  let mut priority = 1;
  for item in 'a'..'z' {
    item_priority_map.insert(item, priority);
    priority += 1;
  }

  for item in 'A'..'Z' {
    item_priority_map.insert(item, priority);
    priority += 1;
  }

  return item_priority_map;
}
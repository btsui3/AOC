use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
pub fn day4() -> String {
  let input = read_data();
  let passports = input.iter()
    .filter_map(|s| parse_passport(s))
    .collect::<Vec<_>>();
   
   return passports.iter()
    .count().to_string();
}


fn read_data() -> HashMap<&str, &str>{
  let values = fs::read_to_string("./src/inputs/input_day04.txt").expect("Could not load file");

  let output = values.split_whitespace()
  .flat_map(|p| p.split(':'))
  .tuples()
  .collect::<HashMap<_,_>>();

  println!("{:?}", output);

  let passport = values.split_whitespace()
  .flat_map(|p| p.split(':'))
  .tuples()
  .collect::<HashMap<_,_>>();

  return output;
}



static VALID_KEYS: [&str; 7] = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];

fn parse_passport(s: &str) -> Option<HashMap<&str,&str>> {
  let passport = s.split_whitespace()
    .flat_map(|p| p.split(':'))
    .tuples()
    .collect::<HashMap<_,_>>();
  if VALID_KEYS.iter().any(|k| !passport.contains_key(k)) {
    return None;
  }
  Some(passport)
}

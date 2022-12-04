use std::fs;
pub fn read_data(input_path: &str) -> String {
  let values = fs::read_to_string(input_path).expect("Could not read file");

  return values;
}
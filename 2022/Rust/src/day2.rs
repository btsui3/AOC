use crate::utility::read_data;

pub fn final_score() -> String {
    let guide = read_data("./src/inputs/day2_input.txt");
    let mut guide_list : Vec<String> = Vec::new();  

    for line in guide.lines() {
      guide_list.push(line.to_string());
  }
// A for Rock, B for Paper, and C for Scissors for competitor 1
// X for Rock, Y for Paper, and Z for Scissors for competitor 2
// A beats Z, B beats X, and C beats Y
// A beats X, B beats Y, and C beats Z
// A beats Y, B beats Z, and C beats X

// If a competitor uses X, they get 1 point
// If a competitor uses Y, they get 2 points
// If a competitor uses Z, they get 3 points
// If a competitor wins, they get 6 points
// If a competitor loses, they get 0 points
// If a competitor ties, they get 3 points

// Calculate score of competitor 2
  let mut score_2 = 0;
  for line in guide_list.iter() {
    let mut line_chars = line.chars();
    let competitor_1 = line_chars.next().unwrap();
    let competitor_2 = line_chars.next().unwrap();
    if competitor_1 == competitor_2 {
      score_2 += 3;
    } else if competitor_1 == 'A' && competitor_2 == 'Z' {
      score_2 += 6;
    } else if competitor_1 == 'B' && competitor_2 == 'X' {
      score_2 += 6;
    } else if competitor_1 == 'C' && competitor_2 == 'Y' {
      score_2 += 6;
    } else if competitor_1 == 'A' && competitor_2 == 'X' {
      score_2 += 1;
    } else if competitor_1 == 'B' && competitor_2 == 'Y' {
      score_2 += 2;
    } else if competitor_1 == 'C' && competitor_2 == 'Z' {
      score_2 += 3;
    } else if competitor_1 == 'A' && competitor_2 == 'Y' {
      score_2 += 0;
    } else if competitor_1 == 'B' && competitor_2 == 'Z' {
      score_2 += 0;
    } else if competitor_1 == 'C' && competitor_2 == 'X' {
      score_2 += 0;
    }
  }
  return score_2.to_string();
}

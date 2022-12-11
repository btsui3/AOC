use crate::utility::read_data;

pub fn final_score() -> String {
    let guide = read_data("./src/inputs/day2_input.txt");
    let mut guide_list : Vec<String> = Vec::new();  

    for line in guide.lines() {
      guide_list.push(line.to_string());
  }

  return part2_score(guide_list);
}

pub fn part2_score(guide_list: Vec<String>) -> String {
  //X -> lose (0 points)
  //Y -> draw (3 points)
  //Z -> win (6 points)
  let mut score_2 = 0;
  for line in guide_list.iter() {
    let mut line_chars = line.chars();
    let competitor_1 = line_chars.next().unwrap();
    let competitor_2 = line_chars.next().unwrap();

    // Calculate score of competitor 2 if competitor 1 uses X using a match statement
    match competitor_1 {
      'A' => {
        match competitor_2 {
          'X' => score_2 += 3 + 0,
          'Y' => score_2 += 1 + 3,
          'Z' => score_2 += 2 + 6,
          _ => (),
        }
      },
      'B' => {
        match competitor_2 {
          'X' => score_2 += 1 + 0,
          'Y' => score_2 += 2 + 3,
          'Z' => score_2 += 3 + 6,
          _ => (),
        }
      },
      'C' => {
        match competitor_2 {
          'X' => score_2 += 2 + 0,
          'Y' => score_2 += 3 + 3,
          'Z' => score_2 += 1 + 6,
          _ => (),
        }
      },
      _ => (),
    }
  }

  return score_2.to_string(); 

}

pub fn part1_score(guide_list: Vec<String>) -> String {
  // A for Rock, B for Paper, and C for Scissors for competitor 1
  // X for Rock, Y for Paper, and Z for Scissors for competitor 2

  // X ->  1 point
  // Y ->  2 points
  // Z ->  3 points

  // Win ->  6 points
  // Lose ->  0 points
  // Tie ->  3 points

  // A beats X and loses to Y and ties with Z
  // B beats Y and loses to Z and ties with X
  // C beats Z and loses to X and ties with Y

  let mut score_2 = 0;
  for line in guide_list.iter() {
    let mut line_chars = line.chars();
    let competitor_1 = line_chars.next().unwrap();
    let competitor_2 = line_chars.next().unwrap();

    // Calculate score of competitor 2 if competitor 1 uses X using a match statement
    match competitor_1 {
      'A' => {
        match competitor_2 {
          'X' => score_2 += 1 + 3,
          'Y' => score_2 += 2 + 6,
          'Z' => score_2 += 3 + 0,
          _ => (),
        }
      },
      'B' => {
        match competitor_2 {
          'X' => score_2 += 1 + 0,
          'Y' => score_2 += 2 + 3,
          'Z' => score_2 += 3 + 6,
          _ => (),
        }
      },
      'C' => {
        match competitor_2 {
          'X' => score_2 += 1 + 6,
          'Y' => score_2 += 2 + 0,
          'Z' => score_2 += 3 + 3,
          _ => (),
        }
      },
      _ => (),
    }
  }

  return score_2.to_string();
}
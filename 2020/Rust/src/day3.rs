use std::fs;

pub fn day3() -> String {
    let rows = read_data();
    let trees = track_path(&rows, 3, 1);
    trees.to_string() 
}


fn read_data() -> Vec<String>{
    let values = fs::read_to_string("./src/inputs/input_day03.txt").expect("Could not load file");

    values
    .split('\n')
    .filter(|&s | !s.is_empty())
    .map(|s|s.to_string())
    .collect()
}

fn track_path(rows: &[String], steps_right: usize, steps_down: usize) -> usize {
    let (trees, _) = rows.iter()
    .skip(steps_down)
    .step_by(steps_down)
    //keep track of position in matrix
    .fold((0usize, 0usize), |(total_trees, pos), s| {
        let (new_pos, is_tree) = move_right(s.as_str(), pos, steps_right);
        let new_total = if is_tree {
            total_trees + 1
        } else {
            total_trees
        };
        (new_total, new_pos)
        
    });
    trees
}
fn move_right(s: &str, pos: usize, steps: usize) -> (usize, bool) {
    let n = s.len();
    let new_pos = (pos + steps) % n;
    let is_tree = char_at(s, new_pos) == '#';
    (new_pos, is_tree)
}

fn char_at(s: &str, pos: usize) -> char {
    s.as_bytes()[pos] as char
}

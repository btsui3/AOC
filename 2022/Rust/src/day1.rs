use crate::utility::read_data;

pub fn sum_of_max_calories() -> String {
    let calories_list = read_data("./src/inputs/day1_input.txt");
    //Read the input file and split it into a Vec<Vec<i32>>. When a blank line is encountered, a new Vec<i32> is created.
    let mut list_of_max_calories: Vec<Vec<i32>> = Vec::new();
    let mut elf_list: Vec<i32> = Vec::new();
    for calories in calories_list.lines() {
        if calories == "" {
          list_of_max_calories.push(elf_list);
            elf_list = Vec::new();
        } else {
          elf_list.push(calories.parse::<i32>().unwrap());
        }
    }
    list_of_max_calories.push(elf_list);

    let mut sum : i32 = 0;
    let mut sum_vec = Vec::new();
    for max_calories in list_of_max_calories {
        sum_vec.push(max_calories.iter().sum::<i32>());
    }

    return sum_vec.iter().max().unwrap().to_string();
}
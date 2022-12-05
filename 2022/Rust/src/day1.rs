use crate::utility::read_data;

pub fn sum_of_max_calories() -> String {
    let list_of_sum_calories = get_list_of_sum_calories();
    
    return list_of_sum_calories.iter().max().unwrap().to_string();
}

pub fn get_list_of_sum_calories() -> Vec<i32> {
    let calories_list = read_data("./src/inputs/day1_input.txt");
    //Read the input file and split it into a Vec<Vec<i32>>. When a blank line is encountered, a new Vec<i32> is created.
    let mut list_of_elf_calories_lists: Vec<Vec<i32>> = Vec::new();
    let mut elf_calories_list: Vec<i32> = Vec::new();
    for calories in calories_list.lines() {
        if calories == "" {
          list_of_elf_calories_lists.push(elf_calories_list);
            elf_calories_list = Vec::new();
        } else {
          elf_calories_list.push(calories.parse::<i32>().unwrap());
        }
    }
    list_of_elf_calories_lists.push(elf_calories_list);

    let mut list_of_sum_calories = Vec::new();
    for calories_list in list_of_elf_calories_lists {
        list_of_sum_calories.push(calories_list.iter().sum::<i32>());
    }
    
    return list_of_sum_calories;
}

pub fn get_top_3_sum() -> String {
    let list_of_sum_calories : Vec<i32>  = get_list_of_sum_calories();
    

    //Return the top 3 sum of calories
    let mut top_3_sum = list_of_sum_calories.clone();
    top_3_sum.sort();
    top_3_sum.reverse();

    return top_3_sum.iter().take(3).sum::<i32>().to_string();}

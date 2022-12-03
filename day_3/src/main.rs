use std::fs;
use std::collections::HashSet;

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents;
}


const LOWER_START: i32 = 'a' as i32;
const LOWER_END: i32 = 'z' as i32;
const UPPER_START: i32 = 'A' as i32;
const UPPER_END: i32 = 'Z' as i32;

fn calculate_priority(letter: char) -> i32{
    let letter_num = letter as i32;
    match letter_num{
        LOWER_START..=LOWER_END =>{
            return letter_num - ('a' as i32) + 1;
        },
        UPPER_START..=UPPER_END =>{
            return letter_num - ('A' as i32) + 27;
        },
        _ =>{
            return -100000;
         }
    }
}

fn problem_1(input: &String) -> i32 {
    // for each line, check split it in half and find the letter that exists in both halfs
    let lines = input.split("\n").filter(|string| string.len()>0);
    let letters: Vec<char> = lines.map(|drawers|{
        // split drawers in half
        assert! (drawers.len() % 2 == 0);
        let left_drawer = &drawers[..drawers.len()/2];
        let right_drawer = &drawers[drawers.len()/2..];
        let left_set: HashSet<char> = left_drawer.chars().collect();
        let right_set: HashSet<char> = right_drawer.chars().collect();
        let intersection = left_set.intersection(&right_set);
        return intersection.copied().into_iter().next().unwrap();
    }).collect();
    let priorities_sum = letters.into_iter().map(|letter|{
        return calculate_priority(letter);
    }).sum();
    return priorities_sum;

}

fn str_to_set(string: &str)->HashSet<char>{
    return string.chars().collect()

}

fn problem_2(input:&String) -> i32{
    // every three lines is a group to find commonality
    let lines: Vec<&str>= input.split("\n").filter(|string| string.len()>0).collect();
    let mut index = 0;
    let mut letters = Vec::new();
    while index < lines.len() {
        let intersection: HashSet<char> = &(&str_to_set(lines[index]) & &str_to_set(lines[index+1])) & &str_to_set(lines[index+2]);
        letters.push(intersection.into_iter().next());
        index +=3;
    }
    let priorities_sum = letters.into_iter().map(|letter|{
        return calculate_priority(letter.unwrap());
    }).sum();
    return priorities_sum;
}


fn main() {
    let input = read_input_file();
    let solution_1 = problem_1(&input);
    println!("Problem1: {:?}", solution_1);
    let solution_2 = problem_2(&input);
    println!("Problem2: {:?}", solution_2);
}

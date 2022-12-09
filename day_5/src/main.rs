/*
[T]     [Q]             [S]        
[R]     [M]             [L] [V] [G]
[D] [V] [V]             [Q] [N] [C]
[H] [T] [S] [C]         [V] [D] [Z]
[Q] [J] [D] [M]     [Z] [C] [M] [F]
[N] [B] [H] [N] [B] [W] [N] [J] [M]
[P] [G] [R] [Z] [Z] [C] [Z] [G] [P]
[B] [W] [N] [P] [D] [V] [G] [L] [T]
 1   2   3   4   5   6   7   8   9 
*/

use std::fs;
use std::cmp;
use regex::Regex;

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents;
}




fn problem_1(mut input_stacks:Vec<Vec<char>>, input: &String) -> Vec<char> {
    let digit_match = Regex::new(r"\d+").unwrap();
    // ok we are going to use the end of the stack as our insertion and removal point
    for instruction in input.split("\n").filter(|string| string.len() >0 ){
        let all_matches: Vec<usize> = digit_match.find_iter(instruction).filter_map(|found_match| found_match.as_str().parse().ok()).collect();
        assert! (all_matches.clone().len() == 3);
        let (number, start, end): (usize,usize, usize) = (all_matches[0], all_matches[1] -1 ,all_matches[2]-1);

        for _ in 0..number{
            if input_stacks[start].len() > 0 {
                let popped_item = input_stacks[start].pop().unwrap();
                input_stacks[end].push(popped_item);
            }
        }

    }
    return input_stacks.iter().filter_map(|row| return row.to_owned().pop()).collect();
}


fn problem_2(mut input_stacks:Vec<Vec<char>>, input: &String) -> Vec<char> {
    let digit_match = Regex::new(r"\d+").unwrap();
    // ok we are going to use the end of the stack as our insertion and removal point
    for instruction in input.split("\n").filter(|string| string.len() >0 ){
        let all_matches: Vec<usize> = digit_match.find_iter(instruction).filter_map(|found_match| found_match.as_str().parse().ok()).collect();
        assert! (all_matches.clone().len() == 3);
        let (number, start, end): (usize,usize, usize) = (all_matches[0], all_matches[1] -1 ,all_matches[2]-1);

        let max_number = cmp::min(input_stacks[start].len(), number);
        let mut to_push = Vec::new();
        for _ in 0..max_number{
            to_push.insert(0,input_stacks[start].pop().unwrap());
        }
        input_stacks[end].extend_from_slice(&to_push);

    }
    return input_stacks.iter().filter_map(|row| return row.to_owned().pop()).collect();
}




fn main() {
    let input_stacks: Vec<Vec<char>> = vec![
        "BPNQHDRT".chars().collect(),
        "WGBJTV".chars().collect(),
        "NRHDSVMQ".chars().collect(),
        "PZNMC".chars().collect(),
        "DZB".chars().collect(),
        "VCWZ".chars().collect(),
        "GZNCVQLS".chars().collect(),
        "LGJMDNV".chars().collect(),
        "TPMFZCG".chars().collect(),
    ];
    let input = read_input_file();
    let solution_1 = problem_1(input_stacks.to_owned(), &input);
    println!("Problem1: {:?}", solution_1);
    let solution_2 = problem_2(input_stacks.to_owned(), &input);
    println!("Problem2: {:?}", solution_2);
}

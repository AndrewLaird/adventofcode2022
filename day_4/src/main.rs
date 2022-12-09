use std::fs;
use std::cmp;
use std::io::{BufRead, BufReader};

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents;
}


fn problem_1(input: &String) -> i32 {
    let ranges: Vec<&str> = input.split("\n").filter(|string| string.len() > 0).collect();
    let result = ranges.iter().map(|elf_ranges|{
        let (elf1,elf2) = elf_ranges.split_once(",").unwrap();
        let (elf1_start, elf1_end) = elf1.split_once("-").unwrap();
        let (elf2_start, elf2_end) = elf2.split_once("-").unwrap();
        let (elf1_start, elf1_end): (i32,i32) = (elf1_start.parse().unwrap(), elf1_end.parse().unwrap());
        let (elf2_start, elf2_end): (i32,i32) = (elf2_start.parse().unwrap(), elf2_end.parse().unwrap());
        if elf1_start <= elf2_start && elf1_end >= elf2_end {
            return 1;
        }
        else if elf2_start <= elf1_start && elf2_end >= elf1_end {
            return 1;
        }
        return 0;
    }).sum();
    return result;
}

fn problem_2(input: &String) -> i32 {
    let ranges: Vec<&str> = input.split("\n").filter(|string| string.len() > 0).collect();
    let result = ranges.iter().map(|elf_ranges|{
        let (elf1,elf2) = elf_ranges.split_once(",").unwrap();
        let (elf1_start, elf1_end) = elf1.split_once("-").unwrap();
        let (elf2_start, elf2_end) = elf2.split_once("-").unwrap();
        let (elf1_start, elf1_end): (i32,i32) = (elf1_start.parse().unwrap(), elf1_end.parse().unwrap());
        let (elf2_start, elf2_end): (i32,i32) = (elf2_start.parse().unwrap(), elf2_end.parse().unwrap());
        if elf2_end < elf1_start || elf1_end < elf2_start {
            return 0;
        }
        return 1;
    }).sum();
    return result;
}

/*
 * I asked chatgpt to optimize the above code this is what it returned
 * https://chat.openai.com/chat
 *
 * small adjustments had to be made but it gave the correct answer
 * input was:

    Can you optimize this rust program:

    file input Input will look like this:
    src/input.txt:
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
 *
 * and then the rust above
 */

fn parse_range(range: &str) -> (i32, i32) {
    let mut parts = range.split('-');
    let start = parts.next().unwrap().parse::<i32>().unwrap();
    let end = parts.next().unwrap().parse::<i32>().unwrap();
    (start, end)
}

fn problem_3(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let (start1, end1) = parse_range(ranges[0]);
        let (start2, end2) = parse_range(ranges[1]);
        if start1 <= start2 && end1 >= end2 {
            result += 1;
        } else if start2 <= start1 && end2 >= end1 {
            result += 1;
        }
    }
    result
}

fn problem_4(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let (start1, end1) = parse_range(ranges[0]);
        let (start2, end2) = parse_range(ranges[1]);
        if end2 < start1 || end1 < start2 {
            result += 0;
        }
        else{
            result += 1;
        }
    }
    result
}



fn main() {
    let input = read_input_file();
    let solution_1 = problem_1(&input);
    println!("Problem1: {:?}", solution_1);
    let solution_2 = problem_2(&input);
    println!("Problem2: {:?}", solution_2);
    let solution_3 = problem_3(&input);
    println!("Problem3: {:?}", solution_3);
    let solution_4 = problem_4(&input);
    println!("Problem4: {:?}", solution_4);

}

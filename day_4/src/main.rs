use std::fs;
use std::cmp;

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



fn main() {
    let input = read_input_file();
    let solution_1 = problem_1(&input);
    println!("Problem1: {:?}", solution_1);
    let solution_2 = problem_2(&input);
    println!("Problem2: {:?}", solution_2);

}

use std::fs;
use std::collections::HashSet;

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents;
}


fn problem_1(input: &String) -> i32 {
    for i in 0..input.len()-4{
        let subset = input.get(i..i+4).unwrap();
        let set: HashSet<char> = subset.chars().collect();
        if set.len() == 4 {
            return (i+4) as i32;
        }
    }
    return -1;
}

fn problem_2(input: &String) -> i32 {
    for i in 0..input.len()-14{
        let subset = input.get(i..i+14).unwrap();
        let set: HashSet<char> = subset.chars().collect();
        if set.len() == 14 {
            return (i+14) as i32;
        }
    }
    return -1;
}



fn main() {
    let input = read_input_file();
    let solution_1 = problem_1(&input);
    println!("Problem1: {:?}", solution_1);
    let solution_2 = problem_2(&input);
    println!("Problem2: {:?}", solution_2);

}

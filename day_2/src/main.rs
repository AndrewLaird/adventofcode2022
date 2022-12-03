use std::fs;
use std::collections::HashMap;

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents;
}


fn who_won(elf_diff: i32) -> i32{
    return vec![3,0,6][elf_diff as usize];
}

fn problem_1(input: &String) -> i32{

    let outcomes: HashMap::<&str, i32> = HashMap::from([
        ("A", 0),
        ("B", 1),
        ("C", 2),
        ("X", 0),
        ("Y", 1),
        ("Z", 2),
    ]);
    // parse into rows
    let rows = input.split("\n").filter(|string| string.len() > 0);
    // for each row calculate the score
    let result = rows.map(|row|{
        let (elf1, elf2) = row.split_once(" ").unwrap();
        let &elf1_num = outcomes.get(elf1).expect("not found");
        let &elf2_num = outcomes.get(elf2).unwrap();
        let winning_points =  who_won((elf1_num - elf2_num).rem_euclid(3));
        return winning_points + elf2_num + 1;
    }).reduce(|a,b| a+b).unwrap();
    return result;

}
fn problem_2(input: &String) -> i32{

    let outcomes: HashMap::<&str, i32> = HashMap::from([
        ("A", 0),
        ("B", 1),
        ("C", 2),
        ("X", -1),
        ("Y", 0),
        ("Z", 1),
    ]);
    // parse into rows
    let rows = input.split("\n").filter(|string| string.len() > 0);
    // for each row calculate the score
    let result = rows.map(|row|{
        let (elf1, elf2) = row.split_once(" ").unwrap();
        let &elf1_num = outcomes.get(elf1).expect("not found");
        let elf2_num = (elf1_num + outcomes.get(elf2).unwrap()).rem_euclid(3);
        println!("{:?}, {:?}, {:?}, {:?}", elf1_num, elf2_num, elf2, who_won((elf1_num - elf2_num).rem_euclid(3)));
        let winning_points =  who_won((elf1_num - elf2_num).rem_euclid(3));
        return winning_points + elf2_num + 1;
    }).reduce(|a,b| a+b).unwrap();
    return result;

}





fn main() {
    let input: String = read_input_file();
    let solution_1 = problem_1(&input);
    let solution_2 = problem_2(&input);
    println!("{:?}",solution_1);
    println!("Solution 2{:?}",solution_2);
}

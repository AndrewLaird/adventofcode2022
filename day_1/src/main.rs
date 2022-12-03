use std::fs;

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents
}


fn problem_1(input: &String) -> i32{
    let elfs = input.split("\n\n");
    let mut max = 0;
    for elf in elfs.into_iter(){
        let sum: i32 = elf.split('\n').filter(|split| split.len() >0).map(|inventory| inventory.parse::<i32>().unwrap()).sum();
        if sum > max {
            max = sum;
        }
    }
    return max;
}

fn problem_2(input: &String) -> Vec<i32>{
    let elfs = input.split("\n\n");
    let mut top_three: Vec<i32> = vec![-2,-1,0];
    for elf in elfs.into_iter(){
        let sum: i32 = elf.split('\n').filter(|split| split.len() >0).map(|inventory| inventory.parse::<i32>().unwrap()).sum();
        let position = top_three.binary_search(&sum);
        match position{
            Ok(position) => {
                // this sum exists already
                // insert after and delete the lowest
                top_three.insert(position, sum);
                top_three.remove(0);
            }
            Err(0) => {
            }
            Err(position) => {
                // not found but should be inserted here
                top_three.insert(position, sum);
                top_three.remove(0);
            }

        }
    }
    return top_three;
}




fn main() {
    let input = read_input_file();
    let result1 = problem_1(&input);
    let result2 = problem_2(&input);
    println!("Problem1 {:?}", result1);
    println!("Problem2 {:?}", result2);
    println!("Problem2 sum {:?}", result2.iter().sum::<i32>());
}

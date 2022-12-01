use std::fs;

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents
}

fn count_increasing(readings: &Vec<i32>) -> i32{
    let mut count: i32 = 0;
    for i in 1..readings.len(){
        if readings[i] > readings[i-1] {
            count+=1;
        }

    }
    return count;
}

fn count_sliding_window(readings: &Vec<i32>) -> i32{
    // hey lets be smart, with the sliding window, the only thing that is changing is the first and
    // last ones
    let mut count: i32 = 0;
    for i in 3..readings.len(){
        if readings[i] > readings[i-3] {
            count+=1;
        }
    }
    return count;


}

fn main() {
    let input = read_input_file();
    let readings: Vec<i32> = input.split("\n").filter(|item| item.len() >0).map(|num| num.parse::<i32>().unwrap()).collect();
    let count = count_increasing(&readings);
    let count2 = count_sliding_window(&readings);
    println!("Answer: {:?}", count);
    println!("Answer: {:?}", count2);
}

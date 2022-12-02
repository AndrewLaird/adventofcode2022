use std::fs;

fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents
}

fn process_input_return_delta(input:String)->Vec<(i32, i32)>{
    let deltas = input.split("\n").filter(|data| data.len() > 0).map(|row|{
        let (direction,distance) = row.split_once(" ").unwrap();
        if direction == "forward" {
            return (distance.parse::<i32>().unwrap(), 0 as i32);
        }
        if direction == "down" {
            return (0 as i32, distance.parse::<i32>().unwrap());
        }
        else{
            return (0 as i32, -distance.parse::<i32>().unwrap());
        }
    }).collect();
    return deltas;
}

fn part_two(deltas: &Vec<(i32,i32)>) -> i32{
    let mut aim = 0;
    let result = deltas.iter().copied().reduce(|(sum_x, sum_y),(forward, horizontal)|{
        if forward != 0 {
            return (sum_x + forward, sum_y + forward*aim);
        }
        else{
            aim += horizontal;
            return (sum_x,sum_y);
        }
    }).unwrap();
    println!("Part Two: {:?}", result.0*result.1);
    return result.0*result.1;

}

fn part_one(deltas: &Vec<(i32,i32)>) -> i32{
    let result: (i32, i32) = deltas.iter().copied().reduce(|sum, current| {
        return (sum.0+current.0, sum.1+current.1);
    }).unwrap();
    println!("Part One: {:?}", result.0*result.1);
    return result.0*result.1;
}


fn main() {
    let data = read_input_file();
    let deltas = process_input_return_delta(data);
    part_one(&deltas);
    part_two(&deltas);
}

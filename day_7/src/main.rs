use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
fn read_input_file()->String{
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents;
}



// Definition of a graph node
#[derive(Debug)]
struct Node {
    directory_name: String, 
    file_total: i32,
    directories: Vec<Rc::<Node>>,
    parent: Rc::<Option<Node>>,
}

impl Node{
    fn new(directory_name: String, parent: Rc<Option<Node>> ) -> Self {
        Node {
            directory_name,
            file_total: 0,
            directories: vec![],
            parent,
        }
    }
    fn add_file(mut self, size: i32){
        self.file_total += size;
    }
}


fn problem_1(input: &String) -> i32 {
   let commands: Vec<Vec<&str>> = input.split("\n").filter(|string| string.len() > 0).map(|x| x.split(" ").collect()).collect();
   let mut current_location = Rc::new(Some(Node::new("/".to_string(), Rc::new(None))));
   let _head = current_location.clone();
    
    for command in commands{
        match command[0]{
            "$" => {
                match command[1] {
                    "cd" => {
                        match command[2] {
                            ".." => {
                                current_location = current_location.as_ref().unwrap().parent;
                                []
                                
                            }
                            _ => {
                            }
                        }
                    }
                    "ls" => {}
                    _ => {}
                }

            }
            _ =>
            {
                match command[1] {
                     "dir" => {}
                     _ => {
                     }

                }

            }
            
        }
    }
    return 5;
}



fn main() {
    let input = read_input_file();
    let solution_1 = problem_1(&input);
    println!("Problem1: {:?}", solution_1);
}

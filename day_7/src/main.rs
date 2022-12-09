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
    directories: Vec<Node>,
    parent: Rc::<Option<mut Node>>,
}

impl Node{
    fn new(directory_name: String, parent: Rc<Option<mut Node>> ) -> Self {
        Node {
            directory_name,
            file_total: 0,
            directories: vec![],
            parent,
        }
    }
    fn bidirectonal_add(mut parent: Node, child: Node){
        // make a child node and push it
        parent.directories.push(child);
    }
    fn add_file(mut self, size: i32){
        self.file_total += size;
    }
}


fn problem_1(input: &String) -> i32 {
   let commands: Vec<Vec<&str>> = input.split("\n").filter(|string| string.len() > 0).map(|x| x.split(" ").collect()).collect();
   let mut current_location = Rc::new(Some(Node::new("/".to_string(), Rc::new(None))));
   let _head = Rc::new(current_location.as_mut());
    
    for command in commands{
        match command[0]{
            "$" => {
                match command[1] {
                    "cd" => {
                        match command[2] {
                            ".." => {
                                current_location = current_location.unwrap().parent;
                                
                            }
                            _ => {
                                let parent = current_location.as_mut().unwrap();
                                let child = Node::new(command[2].to_string(), Rc::new(Some(parent)));
                                parent.directories.push(child);
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
                         current_location.unwrap().add_file(command[1].parse().unwrap());
                         println!("{:?}", current_location);
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

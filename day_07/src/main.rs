use std::{fs, collections::HashMap};

//=========================== NODE ==================================//

struct Node{
    local_total: u64,
    files: Vec<u64>,
    childs: HashMap<String, Node>,
    father: Option<*mut Node>
}

//********************** NODE - METHODS ************************//

impl Node{

    fn new(father: Option<*mut Node>) -> Node{
        Node{
            local_total: 0,
            files: Vec::new(), 
            childs: HashMap::new(),
            father: father
        }
    }
    
    //calculates the nodes total size
    fn calculate_total(&mut self) -> u64{
        let mut total: u64 = self.files_sum();

        for (_, child) in self.childs.iter_mut(){
            total += child.calculate_total();
        }

        self.local_total = total;
        total
    }
    
    //calculates the total size ocuppied by the nodes files
    fn files_sum(&self) -> u64{
        let mut sum: u64 = 0;

        for value in self.files.iter(){
            sum += value;
        }

        sum
    }

    //returns the sum of the size of all nodes whose size is at most comp in a tree
    fn sum_smaller_than(&self, comp: u64)-> u64{
        let mut total: u64 = 0;

        for (_, node) in self.childs.iter(){
            total += node.sum_smaller_than(comp);
        }

        if self.local_total <= comp{
            total += self.local_total;
        }

        total

    }

    //given a size needed to be deleted, returns the size os smallest directory that is bigger
    //than the given value
    fn size_to_delete(&self, comp: u64) -> u64{
        let mut size: u64 = u64::MAX;

        if self.local_total > comp{
            size = self.local_total;
        }

        for (_, node) in self.childs.iter(){
            let node_size: u64 = node.size_to_delete(comp);

            if node_size >= comp && node_size < size{
                size = node_size;
            }
        }

        size
    }

}

//======================================== END - NODE ===================================//

//=========================== ACTION ==================================//
//
enum Action{
    CreateDir(String),
    CreateFile(u64),
    JumpUp,
    GoDown(String),
    None
}

//============================== END - ACTION ==========================//



//======================================== FUNCTIONS ===================================//


fn parse_line(line: &str) -> Action{
    let mut command: Vec<&str> = line.split(' ').collect(); 
    command.retain(|&z| z != "$" && z != "");

    match command[0]{
        "cd" => {
            if command[1] == ".."{
                Action::JumpUp 
            }
            else{
                Action::GoDown(command[1].to_string())
            }
        },

        "dir" => Action::CreateDir(command[1].to_string()),

        "ls" => Action::None,

        _ => Action::CreateFile(command[0].parse().unwrap_or_default())
    }

}


//======================================== END - FUNCTIONS ===================================//


//======================================== MAIN ===================================//

fn main() {
    let file_contents: String = fs::read_to_string("./src/day_seven.txt").unwrap();
    let mut lines: Vec<&str> = file_contents.split("\n").collect();
    lines.retain(|&z| z !="" && z != "$ cd /");

    //creates the filesystem's top level entity
    let mut top_level: Node = Node::new(None);
    let mut current_node: *mut Node = &mut top_level;

    //Creates the Directory Tree based on the commands extracted from the file
    for line in lines.iter(){
        
        //Needs to be unsafe. Otherwise, it would not be possible to process the JumpUp command,
        //since Rust Does not allow multiple simultaneous mutable references.
        unsafe{
            match parse_line(line){
                Action::JumpUp => {
                    current_node = (*current_node).father.unwrap();
                },

                Action::CreateDir(dir_name) =>{
                    (*current_node).childs.insert(dir_name, Node::new(Some(current_node)));
                },

                Action::CreateFile(size) =>{
                    (*current_node).files.push(size);
                },

                Action::GoDown(dir_name) =>{
                    current_node = (*current_node).childs.get_mut(&dir_name).unwrap();
                },

                Action::None => {}

            }
        }

    }

    let total_size = top_level.calculate_total();
    let sum: u64 = top_level.sum_smaller_than(100_000);
    
    let needed_size: u64 = 30_000_000 - (70_000_000 - total_size);
    let to_delete: u64 = top_level.size_to_delete(needed_size);


    println!("Sum: {sum}\nTo delete: {to_delete}");

}

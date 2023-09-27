use std::{fs};

fn main() {
    let file_contents = fs::read_to_string("./src/day_one.txt")
                               .expect("Failed reading file to string");

    let values: Vec<&str> = file_contents.split("\n").collect();
    let mut elves: Vec<i64> = Vec::new();

    let mut cur_elf: i64 = -1;
    
    for value in values.iter(){
        if value.to_string() == "".to_string() && cur_elf != -1{
            elves.push(cur_elf); 
            cur_elf = 0;
        }
        else{
            cur_elf += value.to_string().parse::<i64>().unwrap_or(0);
        }
    }
    
    elves.sort();
    
    let mut total: i64 = 0;

    for i in 0..3{
        total += elves[elves.len() - 1 - i]
    }
    dbg!(elves);
    println!("Top Three: {total}");
}

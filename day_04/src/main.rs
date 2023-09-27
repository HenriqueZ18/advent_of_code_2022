use std::fs;

fn parse_line(pair: &str) -> Option<(u32, u32, u32, u32)>{
    let numbers: Vec<&str> = pair.split([',', '-']).collect();

    if numbers.len() == 4{
        Some(
            (numbers[0].parse().unwrap_or(0), numbers[1].parse().unwrap_or(0),
             numbers[2].parse().unwrap_or(0), numbers[3].parse().unwrap_or(0))) 
    }
    else{
        None
    }
}

fn is_subset(limits: (u32, u32, u32, u32)) -> bool{
    (limits.0 <= limits.2 && limits.1 >= limits.3) || (limits.2 <= limits.0 && limits.3 >= limits.1)
}

fn overlap(limits: (u32, u32, u32, u32)) -> bool{
    (limits.1 >= limits.2  && limits.2 >= limits.0) || (limits.3 >= limits.0 && limits.0 >= limits.2 ) 
}

fn main() {
    let file_contents = fs::read_to_string("./src/day_four.txt").unwrap();
    
    let pairs: Vec<&str> = file_contents.split("\n").collect();
    
    let mut total: u64 = 0;

    for pair in pairs{
        match parse_line(pair){
            Some(tuple) =>{
                println!();
                dbg!(&tuple);
                if overlap(tuple){
                    println!("TRUE");
                    total += 1;
                }
            },
            None => {}
        }
    }

    println!("TOTAL: {total}");

}

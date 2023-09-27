use std::fs;

fn find_wrong_type(comp_one: &[u8], comp_two: &[u8]) -> u64{
    let alphabet: &[u8] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    for i in 0..alphabet.len(){
        if comp_one.contains(alphabet.get(i).unwrap()) && comp_two.contains(alphabet.get(i).unwrap()){
            return (i + 1) as u64;
        }
    }

    0
}

fn find_badge(first: &str, second: &str, third: &str) -> u64{
    let alphabet: &[u8] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

    println!("1 : {first}\n2 : {second}\n3 : {third}");

    let first: &[u8] = first.as_bytes();
    let second: &[u8] = second.as_bytes();
    let third: &[u8] = third.as_bytes();
    
    for i in 0..alphabet.len(){
       if first.contains(&alphabet[i]) && second.contains(&alphabet[i]) && third.contains(&alphabet[i]){
            println!("common: {}\n", alphabet[i] as char);
            return (i + 1) as u64;
       }
    }
    
    println!("NOT FOUND\n");
    0
}

fn main() {
    let file_contents = fs::read_to_string("./src/day_three.txt").unwrap();

    let rucksacks: Vec<&str> = file_contents.split("\n").collect();
    let mut total: u64 = 0;
    
    //dbg!(&rucksacks);
    let mut i: usize = 0;
    while i < rucksacks.len() - 2{
       total += find_badge(rucksacks[i], rucksacks[i + 1], rucksacks[i + 2]);
       i += 3;
    }

    println!("Total: {total}");
}

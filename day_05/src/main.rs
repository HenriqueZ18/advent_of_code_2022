use std::fs;

fn parse_stacks(stacks_lines: &[&str]) -> Vec<Vec<u8>>{
    let mut stacks: Vec<Vec<u8>> = Vec::new();

    let n: usize =(stacks_lines[0].len() -2)/4 + 1; 
    
    //initializing stacks
    for _ in 0..n{
        stacks.push(Vec::new());
    }
    
    for line in stacks_lines.iter().rev(){
        let line_bytes: &[u8] = line.as_bytes();    
        
        //Find the stacks that have an element 
        for k in 0..n{
            if line_bytes[1 + 4*k] != b' '{
                stacks[k].push(line_bytes[1 + 4*k]);
            }
        }
    }

    stacks
}

fn parse_moves(moves: &[&str]) -> Vec<(usize, usize, usize)>{
    let mut moves_tuples: Vec<(usize, usize, usize)> = Vec::new();

    for mv in moves.iter(){
        let mut values: Vec<&str> = mv.split(['m', 'o', 'v', 'e', 'f', 'r', 't', ' ']).collect();
        values.retain(|z| *z != "");
        moves_tuples.push((values[0].parse().unwrap(),values[1].parse().unwrap(), values[2].parse().unwrap()))
    }

    moves_tuples
}

fn make_moves_pone(stacks: &mut Vec<Vec<u8>>, moves: &Vec<(usize, usize, usize)>){
    for mv in moves.iter(){
        for _i in 0..mv.0{
            let popped: u8 = stacks[mv.1 - 1].pop().unwrap();
            stacks[mv.2 -1].push(popped);
        }
    }
}

fn make_moves_ptwo(stacks: &mut Vec<Vec<u8>>, moves: &Vec<(usize, usize, usize)>){
    for mv in moves.iter(){
        let length: usize = stacks[mv.1 - 1].len();
        let mut drained: Vec<u8> = stacks[mv.1 -1].drain((length - mv.0)..).collect();
        stacks[mv.2 - 1].append(&mut drained);
    }
}

fn make_moves_pone_v2(stacks: &mut Vec<Vec<u8>>, moves: &Vec<(usize, usize, usize)>){
    for mv in moves.iter(){
        let length: usize = stacks[mv.1 - 1].len();
        let mut drained: Vec<u8> = stacks[mv.1 -1].drain((length - mv.0)..).rev().collect();
        stacks[mv.2 - 1].append(&mut drained);
    }
}

fn get_top_boxes(stacks: &Vec<Vec<u8>>) -> String{
    let mut top_boxes: String = String::new();
    
    for i in 0..stacks.len(){
        let top: usize = stacks[i].len() - 1;

        top_boxes.push(stacks[i][top] as char);
    }


    top_boxes
}

fn print_stacks(stacks: &Vec<Vec<u8>>){

    println!("\n=================== STACKS ==================\n");
    for i in 0..stacks.len(){
        print!("\n{}: ", i + 1);

        for value in stacks[i].iter(){
            print!("{} ", *value as char);
        }
    }
    println!("\n=========================================\n")
}

fn main() {
    let file_contents = fs::read_to_string("./src/day_five.txt").unwrap();

    let mut lines: Vec<&str> = file_contents.split("\n").collect();
    lines.retain(|x| *x != "");
    
    let mut stacks_end: usize = 0;
    
    for line in lines.iter(){
        if line.contains('1'){
            break;
        }
        stacks_end += 1;
    }
    
    let mut stacks: Vec<Vec<u8>> = parse_stacks(&lines[..stacks_end]);
    let moves: Vec<(usize, usize, usize)> = parse_moves(&lines[(stacks_end + 1)..]);
    
    print_stacks(&stacks);

    make_moves_pone_v2(&mut stacks, &moves);

    print_stacks(&stacks);

    println!("\n\nTOP BOXES: {}", get_top_boxes(&stacks));

    
}

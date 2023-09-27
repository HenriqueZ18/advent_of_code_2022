use std::fs;

enum Op{
    Add(i32),
    Noop
}

fn parse_line(line: &str) -> Op{
    let mut line: Vec<&str> = line.split(' ').collect();
    line.retain(|z| *z != " " && *z != "");

    if line[0] == "addx"{
        return Op::Add(line[1].parse().unwrap());
    }

    Op::Noop
}

fn main() {
    let file_contents: String = fs::read_to_string("./src/day_ten.txt").unwrap();

    let mut lines = file_contents.lines();

    let mut op_len: u8 = 1;
    let mut aux: i32 = 0;
    let mut x: i32 = 1;
    let mut i: u8 = 1; 

    for clocks in 1..=240{
        if i == op_len{
            x += aux;
            i = 0;
            let line = lines.next().unwrap_or("noop");

            match parse_line(line){
                Op::Add(imm) =>{
                    op_len = 2;
                    aux = imm;
                },
                Op::Noop => {
                    op_len = 1;
                    aux = 0;
                }
            }
        }
        i += 1;

        if (clocks - 1)%40 == 0{
            print!("\n");
        }
        if (clocks - 1) % 40 >= x - 1 && (clocks - 1) % 40 <= x + 1{
            print!("#");
        }
        else{
            print!(".");
        }

    }
    
}

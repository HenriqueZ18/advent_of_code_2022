use std::fs;

#[derive(Debug)]
enum Op {
    Division(i32),
    Multiplication(i32),
    Sum(i32),
    Subtraction(i32),
    Noop
}

#[derive(Debug)]
struct Monkey{
    id: usize,
    items: Vec<Vec<u32>>,
    op: Op,
    test: Op,
    receivers: (i32, i32),
    activity: u32 
}

impl Monkey{
    fn new() -> Monkey{
        Monkey{
            id: 0,
            items: Vec::new(),
            op: Op::Noop,
            test: Op::Noop,
            receivers: (0, 0),
            activity: 0
        }
    }

    fn play(&mut self, tests: &[i32]) -> Vec<(Vec<u32>, i32)>{
        let mut plays: Vec<(Vec<u32>, i32)> = Vec::with_capacity(self.items.len());
            
        for j in (0..self.items.len()).rev(){
            match self.op{
                Op::Multiplication(operand) =>{
                    if operand >=0{
                        for i in 0..self.items[j].len(){
                            self.items[j][i] = ((self.items[j][i] as i32 * operand) % tests[i]) as u32;
                        }
                    }
                    else{
                        for i in 0..self.items[j].len(){
                            self.items[j][i] = (self.items[j][i] * self.items[j][i]) % tests[i] as u32;
                        }
                    }
                },
                Op::Sum(operand) =>{
                    if operand >=0{
                        for i in 0..self.items[j].len(){
                            self.items[j][i] = (self.items[j][i] + operand as u32) % tests[i] as u32;
                        }
                    }
                    else{
                        for i in 0..self.items[j].len(){
                            self.items[j][i] =(self.items[j][i] + self.items[j][i]) % tests[i] as u32;
                        }
                    }
                },
                _ => {} 
            };

            //new = ((new as f64 + 0.5)/3.0) as i32;
            
            let receiver: i32 ={
                if self.items[j][self.id] == 0{
                    self.receivers.0 
                }
                else{
                    self.receivers.1
                }
            };
            
            plays.push((self.items.remove(j), receiver));
        }
        
        self.activity += plays.len() as u32;
        plays
    }
}

fn parse_items(items: &str, monkeys: usize) -> Vec<Vec<u32>>{
    let mut line_zero: Vec<&str> = items.split([' ', ',']).collect();
    line_zero.retain(|&z| z != "" && z != "Starting" && z != "items:");

    let mut return_vec: Vec<Vec<u32>> = Vec::with_capacity(line_zero.len());
    for &item in line_zero.iter(){
        match item.parse::<u32>(){
            Err(_) => {},
            Ok(value) => {
                return_vec.push(vec![value; monkeys]);
            }
        }
    }
    
    return_vec
}

fn parse_operation(line: &str) -> Op{
    let mut splitted_line: Vec<&str> = line.split(' ').collect();
    splitted_line.retain(|&x| x != "Operation:" && x != "=" && x != "" && x != "new");
    
    if splitted_line.len() == 3{
        let operand: i32 = splitted_line[2].parse().unwrap_or(-1);

        match splitted_line[1] {
            "*" => Op::Multiplication(operand),
            "+" => Op::Sum(operand),
            "/" => Op::Division(operand),
            "-" => Op::Subtraction(operand),
            _ => Op::Noop
        }
    }
    else{
        Op::Noop
    }
}

fn parse_test(line: &str) -> Op{
   let mut splitted_line: Vec<&str> = line.split(' ').collect();
   splitted_line.retain(|&z| z != "");

   Op::Division(splitted_line[3].parse::<i32>().unwrap_or(-1))

}

fn parse_receivers(lines: &[&str]) -> (i32, i32){
    let mut receivers: [i32 ; 2] = [0 ; 2];
    for i in 0..lines.len(){
        let mut splitted_line: Vec<&str> = lines[i].split(' ').collect();
        splitted_line.retain(|&z| z != "");
        receivers[i] = splitted_line[5].parse().unwrap_or(-1);
    }

    (receivers[0], receivers[1])
}

fn parse_input(input: String) -> Vec<Monkey>{
    let monkeys_string: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys_vec: Vec<Monkey> = Vec::with_capacity(monkeys_string.len());
    let mut monkey_id: usize = 0;

    for monkey in monkeys_string.iter(){
        let mut lines: Vec<&str> = monkey.lines().collect();
        lines.retain(|z| z.contains("Starting") || z.contains("Operation") || z.contains("Test") || z.contains("If"));
        
        let mut cur_monkey: Monkey = Monkey::new();
        
        cur_monkey.id = monkey_id;
        cur_monkey.items = parse_items(lines[0], monkeys_string.len());
        cur_monkey.op = parse_operation(lines[1]);
        cur_monkey.test = parse_test(lines[2]);
        cur_monkey.receivers = parse_receivers(&lines[3..5]);

        monkeys_vec.push(cur_monkey);

        monkey_id += 1;
    }
    
    monkeys_vec

}

fn main() {
    let file_contents: String = fs::read_to_string("./src/day_eleven.txt").unwrap();
    let mut monkeys: Vec<Monkey> = parse_input(file_contents);
    let mut tests: Vec<i32> = Vec::with_capacity(monkeys.len());

    for monkey in monkeys.iter(){
        match monkey.test{
            Op::Division(div) =>{
                tests.push(div);
            },
            _ => {}
        }
    }

    for k in 0..10_000{
        println!("======= ROUND {} ===========", k);
        for i in 0..monkeys.len(){
            let cur_play: Vec<(Vec<u32>, i32)> = monkeys[i].play(&tests); 
            monkeys[i].items.clear();

            for (new, rec) in cur_play{
                if rec >= 0 && new.len() > 0{
                    monkeys[rec as usize].items.push(new);
                }
            }
            println!("{} : {}", i, monkeys[i].activity);

        }
    }
    
    let mut monkey_bus: Vec<u32> = Vec::with_capacity(monkeys.len());

    for monkey in monkeys.iter(){
        monkey_bus.push(monkey.activity);    
    }

    monkey_bus.sort_unstable();

    dbg!(&monkey_bus);
    println!("{}", monkey_bus[monkey_bus.len() - 1] as u128 * monkey_bus[monkey_bus.len() - 2] as u128);
}

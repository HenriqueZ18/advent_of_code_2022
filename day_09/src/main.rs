use std::{fs, num};

enum Direction{
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

struct Coord{
    x: i32,
    y: i32,
}

impl Coord{
    
    fn new() -> Coord{
        Coord{x: 0, y: 0}
    }

    fn sq_distance(&self, point: (i32, i32)) -> u32{
       u32::try_from(i32::pow(self.x - point.0, 2) + i32::pow(self.y - point.1, 2)).unwrap()
    }

    fn move_adj(&mut self, point: (i32, i32)){
        //Analytic Geometry: AB = B - A

        let mut dis_x: i32 = point.0 - self.x;
        
        if dis_x != 0{
            dis_x = dis_x/dis_x.abs();
        }

        let mut dis_y: i32 = point.1 - self.y;

        if dis_y != 0{
            dis_y = dis_y/dis_y.abs();
        }


        self.x += dis_x;
        self.y += dis_y;
    }

    fn as_tuple(&self) -> (i32, i32){
        (self.x, self.y)
    }
}

fn parse_line(line: &str) -> Option<Direction>{
    let mut line: Vec<&str> = line.split(' ').collect();
    line.retain(|x| *x != " " && *x != "");
    let steps: u32 = line[1].parse().unwrap();

    match line[0] {
        "U" => Some(Direction::Up(steps)),
        "D" => Some(Direction::Down(steps)),
        "L" => Some(Direction::Left(steps)),
        "R" => Some(Direction::Right(steps)),
        _ => None
    }
}

fn move_rope(snake: &mut Vec<Coord>, positions: &mut Vec<(i32, i32)>, dir: Direction){
    let mult_x: i32;
    let mult_y: i32;
    let steps: u32;

    match dir{
        Direction::Up(s) => {
            mult_x = 0;
            mult_y = 1;
            steps = s;
        },
        Direction::Down(s) => {
            mult_x = 0;
            mult_y = -1;
            steps = s;
        },
        Direction::Left(s) =>{
            mult_x = -1;
            mult_y = 0;
            steps = s;
        },
        Direction::Right(s) =>{
            mult_x = 1;
            mult_y = 0;
            steps = s;
        }
    }

    for _ in 0..steps{

        snake[0].x += mult_x;
        snake[0].y += mult_y;
        for i in 1..snake.len(){
            if i32::abs(snake[i - 1].x - snake[i].x) > 1 || i32::abs(snake[i - 1].y - snake[i].y) > 1{
                let prev_node = snake[i - 1].as_tuple();    
                snake[i].move_adj(prev_node);
            }
        }

        if !(positions.contains(&snake.last().unwrap().as_tuple())){
            positions.push(snake.last().unwrap().as_tuple());
        }

    }

}

fn main() {

    let file_contents: String = fs::read_to_string("./src/day_nine.txt").unwrap();

    let lines: Vec<&str> = file_contents.lines().collect();
    let mut snake: Vec<Coord> = Vec::new();

    for _ in 0..10{
        snake.push(Coord::new());
    }

    let mut positions: Vec<(i32, i32)> = vec![(0, 0)];

    for line in lines{
        let dir: Direction = parse_line(line).unwrap();
        
        move_rope( &mut snake, &mut positions, dir);
    }
    
    dbg!(&positions);
    println!("POS: {}", positions.len());
}

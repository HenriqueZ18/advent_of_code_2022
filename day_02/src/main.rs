use std::fs;

fn calculate_play_part_one(play: &str) -> u64{
    match play{
        "A X" => 4, //rock rock
        "A Y" => 8, //rock paper
        "A Z" => 3, //rock scissors
        "B X" => 1, //paper rock
        "B Y" => 5, //paper paper
        "B Z" => 9, //paper scissors
        "C X" => 7, //scissors rock
        "C Y" => 2, //scissors paper
        "C Z" => 6, //scissors scissors
        _ => 0
    }   
}

fn calculate_play_part_two(play: &str) -> u64{
    match play{
        "A X" => 3, //rock scissors
        "A Y" => 4, //rock rock
        "A Z" => 8, //rock paper
        "B X" => 1, //paper rock
        "B Y" => 5, //paper paper
        "B Z" => 9, //paper scissors
        "C X" => 2, //scissors paper
        "C Y" => 6, //scissors scissors
        "C Z" => 7, //scissors rock
        _ => 0
    }   
}



fn main() {
    let file_contents = fs::read_to_string("./src/day_two.txt")
                                          .expect("Failed Reading from file.");
    
    let plays: Vec<&str> = file_contents.split("\n").collect();
    let mut points: u64 = 0;
    
   for play in plays.iter(){
        points += calculate_play_part_two(play);
   }

   println!("Points: {points}");
}

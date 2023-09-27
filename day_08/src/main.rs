use std::{fs, ops::Range};

fn parse_input(input: String) -> Vec<Vec<u8>>{
    let lines = input.lines();
    let mut i: usize = 0;

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in lines{
       let chars = line.chars();
       grid.push(Vec::new()); 

       for char in chars{
            grid[i].push(u8::try_from(char).unwrap() - 48);  
       }

       i += 1;
    }

    grid
}

fn max(slice: &Vec<Vec<u8>>, range_i: Range<usize>, range_j: Range<usize>) -> u8{
    let mut max: u8 = 0;
    
    let range_i: Vec<usize> = range_i.collect();
    let range_j: Vec<usize> = range_j.collect();

    for &i in range_i.iter(){
        for &j in range_j.iter(){
            max = max.max(slice[i][j]);
        }
    }
    
    max
}

fn scenic_score(matrix: &Vec<Vec<u8>>, i: usize, j : usize) -> u64{
    let mut distances: [u64; 4] = [0;4];
    
    //up
    for k in (0..i).rev(){
        distances[0] += 1;
        if matrix[k][j] >= matrix[i][j]{
            break;
        }
    }
    
    //down
    for l in (i+1)..matrix.len(){ 
        distances[1] += 1;
        if matrix[l][j] >= matrix[i][j]{
            break;
        }
    }

    //left
    for m in (0..j).rev(){
        distances[2] += 1;
        if matrix[i][m] >= matrix[i][j]{
            break;
        }
    }
    
    //right
    for m in (j+1)..matrix[i].len(){
        distances[3] += 1;
        if matrix[i][m] >= matrix[i][j]{
            break;
        }
    }
    dbg!(&distances);
    distances[0]*distances[1]*distances[2]*distances[3]

}

fn main() {
    let file_contents: String = fs::read_to_string("./src/day_eight.txt").unwrap();
    let mut best_scenic: u64= 0;

    let grid: Vec<Vec<u8>> = parse_input(file_contents);
    

    for i in 1..(grid.len() - 1){
        for j in 1..(grid[i].len() - 1){
           best_scenic = best_scenic.max(scenic_score(&grid, i, j)); 
        }
    }

    
    println!("Best Scenic: {best_scenic}");


}

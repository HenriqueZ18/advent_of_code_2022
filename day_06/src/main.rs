use std::fs;

fn find_unique_seq(chars: &Vec<char>, size: usize) -> Option<u32>{
    let mut i: usize = 0;
    let mut temp: Vec<char> = Vec::new();

    while i <= (chars.len() - size){
       for k in 0..size{
            if temp.contains(&chars[i + k]){
                temp.clear();
                break;
            }
            else{
                temp.push(chars[i + k]);
            }
       }

       if !(temp.is_empty()){
            dbg!(&temp);
            return Some(i as u32);
       }

       i += 1;
    }

    None 
}

fn find_unique_seq_v2(chars: &Vec<char>, size: usize) -> Option<u32>{
    let mut i: usize = size;

    let mut counter: u32 = 0;
    
    let mut alphabet: [u8; 26] = [0u8 ; 26];
    for k in 0..size{
        let pos: usize = (u8::try_from(chars[k]).unwrap() - 97) as usize; 
        
        if alphabet[pos] == 0{
            counter += 1;
        }
        else if alphabet[pos] == 1{
            counter -= 1;
        }
        alphabet[pos] += 1;
    }
    
    let mut begin: usize = 0;
    while i < chars.len(){
        let pos_begin: usize = (u8::try_from(chars[begin]).unwrap() - 97) as usize;
        let pos_last: usize = (u8::try_from(chars[i]).unwrap() - 97) as usize;
        

        if alphabet[pos_begin] == 1{
            counter -= 1;
        }
        else if alphabet[pos_begin] == 2{
            counter += 1;
        }
        
        alphabet[pos_begin] -= 1;

        if alphabet[pos_last] == 0{
            counter += 1;
        }
        else if alphabet[pos_last] == 1{
            counter -= 1;
        }
        alphabet[pos_last] += 1;

        if counter == size as u32{
            return Some((begin + 1) as u32);
        }

        i+= 1;
        begin += 1;

    }

    None
     
}

fn find_unique_seq_v3(bytes: &[u8], size: usize) -> Option<usize>{
    let mut alphabet_parity: u32 = 0;
    
    bytes.iter()
         .take(size - 1)
         .for_each(|z| alphabet_parity ^= 1 << (z % 32));

    bytes.windows(size).position(|h| {
        let first = h[0];
        let last = h[h.len() - 1];
        
        alphabet_parity ^= 1 << (last % 32);

        let uniques: bool = alphabet_parity.count_ones() == size as u32;

        alphabet_parity ^= 1 << (first % 32);

        uniques
        })

}

fn find_unique_seq_v3_1(bytes: &[u8], size: usize) -> Option<usize>{
    let mut alphabet_parity: u32 = 0;
    let mut index: usize = size - 1;
    let mut begin: usize = 0;

    for k in 0..(size - 1){
        alphabet_parity ^= 1 << (bytes[k] % 32);
    }

    while index < bytes.len(){
        alphabet_parity ^= 1 << (bytes[index] % 32);

        if alphabet_parity.count_ones() == size as u32{
            return Some(begin);
        }

        alphabet_parity ^= 1 << (bytes[begin] % 32);

        begin += 1;
        index += 1;
    }

    None
    
}

fn main() {
    let mut file_contents = fs::read_to_string("./src/day_six.txt").unwrap();
    file_contents.retain(|z| (z != '\n' && z != ' '));

    let chars: Vec<char> = file_contents.chars().collect();
    
    let bytes = file_contents.as_bytes();
    
    

    
    let pos: u32 = find_unique_seq_v2(&chars, 14).unwrap();

    println!("last: {}", pos + 14);
}

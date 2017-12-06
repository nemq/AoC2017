use std::io::prelude::*; 
use std::io::BufReader; 
use std::fs::File; 
 
pub fn first_puzzle() -> String 
{ 
    let mut jumps = read_jumps("jumps.txt");
    format!("{}", count_steps(&mut jumps, add_one)) 
} 
 
pub fn second_puzzle() ->String 
{ 
    let mut jumps = read_jumps("jumps.txt");
    format!("{}", count_steps(&mut jumps, add_or_sub_one)) 
} 

fn read_jumps(path: &str) -> Vec<i32>
{

    let file = File::open(path).expect("Failed to open jumps file"); 
    let reader = BufReader::new(file); 
    reader.lines() 
          .filter_map(|res| res.ok()) 
          .filter_map(|line| line.parse::<i32>().ok()) 
          .collect::<Vec<i32>>()
}
 
fn count_steps<F: Fn(&mut i32)>(jumps: &mut [i32], adjust_offset: F) -> u32 
{ 
    let mut idx: i32 = 0; 
    let mut count: u32 = 0; 
    while idx >= 0 && idx < jumps.len() as i32 
    { 
        let offset = jumps[idx as usize]; 
        adjust_offset(&mut jumps[idx as usize]);
        idx += offset; 
        count +=1; 
    } 
 
    count 
} 
 
fn add_one(offset: &mut i32)
{
    *offset += 1;
}

fn  add_or_sub_one(offset: &mut i32)
{
    if *offset < 3
    {
        *offset += 1;
    }
    else
    {
        *offset -= 1;
    }
}

#[cfg(test)] 
mod tests 
{ 
    use super::*; 
 
    #[test] 
    fn first_puzzle() 
    { 
        assert_eq!(5, count_steps(&mut [0, 3, 0, 1, -3], add_one)); 
    } 
 
    #[test] 
    fn second_puzzle() 
    { 
        assert_eq!(10, count_steps(&mut [0, 3, 0, 1, -3], add_or_sub_one)); 
    } 
}
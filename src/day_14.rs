use day_10::knot_hash;
use std::ops::Index;
use std::ops::IndexMut;

pub fn first_puzzle() -> String
{
    let disk = Grid::from_key("oundnydw");
    format!("{}", disk.count_used())
}

pub fn second_puzzle() -> String
{
    let disk = Grid::from_key("oundnydw");
    format!("{}", disk.count_regions())
}

fn row_key(key: &str, row: u32) -> String
{
    format!("{}-{}", key, row)
}

fn row(key: &str, row_num: u32) -> Vec<u8>
{
    let mut bytes = Vec::new();
    let row_key = row_key(key, row_num);
    let row_hash = knot_hash(&row_key);
    let mut iter =  row_hash.as_str().chars();
    while let Some(upper_nibble) = iter.next()
    {
        if let Some(lower_nibble) = iter.next()
        {
           let byte = u8::from_str_radix(&format!("{}{}",upper_nibble, lower_nibble), 16)
                         .expect(&format!("Invalid characters: {}{}", upper_nibble, lower_nibble));
           bytes.push(byte);
        }
        else
        {
            panic!("Odd number of nibbles!");
        }
    }
    bytes
}

fn format_row(bytes: &Vec<u8>) -> String
{
    let mut row = String::new();
    for byte in bytes.iter()
    {
        row.push_str(&format!("{:08b}", byte));
    }
    row
}

struct Grid {
    grid: Vec<Vec<bool>>
}

impl Grid {
    fn new() -> Grid {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        let mut row: Vec<bool> = Vec::new();
        row.resize(128, false);
        grid.resize(128, row);
        Grid {grid}
    }

    fn from_key(key: &str) -> Grid {
        let mut disk = Grid::new();
        for i in 0..128 {
            let row_str = format_row(&row(key, i));
            for (j, ch) in row_str.chars().enumerate() {
                if ch == '1' {
                    disk.grid[i as usize][j as usize] = true;
                }
                else {
                    disk.grid[i as usize][j as usize] = false;
                }
            }
        } 
        disk
    }

    fn count_used(self: &Self) -> usize {
        let mut sum = 0;
        for row in self.grid.iter() {
            sum += row.iter().filter(|&&cell| cell).count();
        }
        sum
    }

    fn count_regions(self: &Self) -> usize {
        let mut count = 0;
        let mut used = Grid::new();
        for row in 0..128
        {
            for col in 0..128
            {
                if !used[(row, col)]
                {
                    if self[(row, col)] {
                        self.mark_region((row, col), &mut used);
                        count +=1;
                    }
                    else {
                        used[(row, col)] = true;
                    }
                }
            }
        }
        count
    }

    fn mark_region(self: &Self, (row, col): (usize, usize), used: &mut Grid) {
        used[(row, col)] = true;

        let mut proceed = |idx| { 
            if  !used[idx] {
                if self[idx] {
                    self.mark_region(idx, used);
                }
                else {
                    used[idx] = true;
                }
            }
        };

        if let Some(_) = row.checked_sub(1) {
            proceed((row -1, col));
        }

        if row + 1 < 128 {
            proceed((row + 1, col));
        }
        
        if let Some(_) = col.checked_sub(1) {
            proceed((row, col - 1));
        }

        if col + 1 <128 {
            proceed((row, col + 1));
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;
    fn index(self: &Self, idx: (usize, usize)) -> &bool { 
        let (row, col) =idx;
        &self.grid[row][col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(self: &mut Self, idx: (usize, usize)) -> &mut bool { 
        let (row, col) =idx;
        &mut self.grid[row][col]
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_row_key()
    {
        let key = "flqrgnkx";
        assert_eq!(&row_key(key, 0), "flqrgnkx-0");
        assert_eq!(&row_key(key, 1), "flqrgnkx-1");
        assert_eq!(&row_key(key, 2), "flqrgnkx-2");
    }

    #[test]
    fn test_count_used() {
        let key = "flqrgnkx";
        let disk = Grid::from_key(key);
        assert_eq!(disk.count_used(), 8108);
    }

    #[test]
    fn test_count_regions() {
        let key = "flqrgnkx";
        let disk = Grid::from_key(key);
        assert_eq!(disk.count_regions(), 1242);
    }
}
use std::fs::File;
use std::io::prelude::*;

pub fn first_puzzle() -> String
{
    let sheet = from_file("spreadsheet.txt");
    format!("{}", first_checksum(&sheet))
}

pub fn second_puzzle() -> String
{
    let sheet = from_file("spreadsheet.txt");
    format!("{}", second_checksum(&sheet))
}

type Spreadsheet = Vec<Vec<i32>>;

fn first_checksum(sheet: &Spreadsheet) -> i32
{
    let mut csum = 0;
    for ref row in sheet.iter()
    {
        if let (Some(max), Some(min)) = (row.iter().max(), row.iter().min())
        {
            csum += max - min;
        }
    }

    csum
}

fn second_checksum(sheet: &Spreadsheet) -> i32
{
    let mut sum = 0;
    for row in sheet.iter()
    {
        match find_evenly_dividing_pair(row)
        {
            Some((first, second)) if first > second => {sum += first / second},
            Some((first, second)) if first <= second => {sum += second / first},
            _ => {}
        }
    }
    sum
}

fn find_evenly_dividing_pair(numbers: &[i32]) -> Option<(i32, i32)>
{
    for (idx, &first_num) in numbers.iter().enumerate()
    {
        for &second_num in numbers.iter().skip(idx + 1)
        {
            if first_num % second_num == 0 || second_num % first_num == 0
            {
                return Some((first_num, second_num))
            } 
        }
    }

    None
}

fn from_file(path: &str) -> Spreadsheet
{
    let mut file = File::open(path).expect("Failed to open spreadsheet");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read spreadsheet content");

    from_str(content.as_str())
}

fn from_str(data: &str) -> Spreadsheet
{
    let mut sheet = Spreadsheet::new();
    for line in data.split('\n')
    {
        let row  = line.split_whitespace()
                        .filter_map( |el|
                            el.parse::<i32>().ok())
                        .collect::<Vec<i32>>();
        sheet.push(row);
    }

    sheet
}


#[cfg(test)]
mod tests 
{
    use super::*;
    macro_rules! spreadsheet 
    {
        ( $($( $x:expr ),*);* ) => {
            {
                let mut sheet = Spreadsheet::new();
                $(
                    #[allow(unused_mut)]
                    let mut row = Vec::new();
                    $(
                        row.push($x);
                    )*

                    sheet.push(row);
                )*
                sheet
            }
        };
    }

    #[test]
    fn first_puzzle() 
    {
        let sheet = spreadsheet![
            5, 1, 9, 5;
            7, 5, 3;
            2, 4, 6, 8;
        ];
        assert_eq!(18, first_checksum(&sheet));
    }

    #[test]
    fn second_puzzle()
    {
        assert_eq!(Some((2,8)), find_evenly_dividing_pair(&vec![5, 9, 2, 8]));
        assert_eq!(Some((9,3)), find_evenly_dividing_pair(&vec![9, 4, 7, 3]));
        assert_eq!(Some((3,6)), find_evenly_dividing_pair(&vec![3, 8, 6, 5]));
        let sheet = spreadsheet![5,9,2,8; 9,4,7,3; 3,8,6,5];
        assert_eq!(9, second_checksum(&sheet));
    }
}

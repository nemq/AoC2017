use std::io::prelude::*; 
use std::fs::File; 

pub fn first_puzzle() -> String
{
    let stream = read_stream("stream.txt");
    format!("{}", score(&stream, 1).0)
}

pub fn second_puzzle() -> String
{
    let stream = read_stream("stream.txt");
    format!("{}", score(&stream, 1).1)
}

fn read_stream(path: &str) -> String
{
    let mut file = File::open(path).expect("Failed to open stream file"); 
    let mut stream = String::new();
    file.read_to_string(&mut stream).expect("Failed to read stream file");
    stream
}

fn match_brackets_curly(slice: &str) -> Option<usize>
{
    let mut count = 1;
    let mut ignore = 0;
    for (idx, ch) in slice.chars().enumerate().skip(1)
    {
        match ch
        {
             _  if idx < ignore => {},
            '{'                 => count +=1,
            '}' if count == 1   => return Some(idx),
            '}'                 => count -=1,
            '<'                 => {
                                        if let Some((offset, _)) = match_brackets_angle(&slice[idx..])
                                        {
                                            ignore = idx + offset +1; 
                                        }
                                   }
            _                   => {}
        };
    }

    None
}

fn match_brackets_angle(slice: &str) -> Option<(usize, u32)>
{
    let mut trash = 0;
    let mut ignore = false;
    for (idx, ch) in slice.chars().enumerate().skip(1)
    {
        match ch
        {
             _  if ignore   => ignore = false,
            '!'             => ignore = true,
            '>'             => return Some((idx, trash)),
             _              => trash += 1
        };
    }

    None
}

fn score(stream: &str, point_per_group: u32) -> (u32, u32)
{
    let mut sum = 0;
    let mut trash = 0;
    let mut skip_idx = 0;
    for (idx, ch) in stream.char_indices()
    {
        match (idx, ch)
        {
            (start, '{') if idx >= skip_idx => 
            {
                if let Some(offset) = match_brackets_curly(&stream[start..])
                {
                    skip_idx = start + offset + 1;
                    sum += point_per_group;
                    let (more_sum, more_trash) = score(&stream[start + 1 .. start + offset], point_per_group + 1);
                    sum += more_sum;
                    trash += more_trash;
                } 
                else
                {
                    panic!("unmatched {{: {}", &stream[start..]);
                }
            }, 
            (start, '<') if idx >= skip_idx => 
            {
                if let Some((offset, more_trash)) = match_brackets_angle(&stream[start..])
                {
                    skip_idx = start + offset + 1;
                    trash += more_trash;
                }
                else
                {
                    panic!("unmatched <: {}", &stream[start ..]);
                }
            },
            _ => {}
        }
    }

    (sum, trash)
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_match_brackets_angle() 
    {
        assert_eq!(Some((1, 0)), match_brackets_angle("<>"));
        assert_eq!(Some((2, 1)), match_brackets_angle("<a>"));
        assert_eq!(Some((4, 3)), match_brackets_angle("<<<<>"));
        assert_eq!(Some((5, 2)), match_brackets_angle("<{!>}>"));
        assert_eq!(Some((3, 0)), match_brackets_angle("<!!>"));
        assert_eq!(Some((5, 0)), match_brackets_angle("<!!!>>"));
        assert_eq!(Some((13, 10)), match_brackets_angle("<{o\"i!a,<{i<a>"));
    }

    #[test]
    fn test_match_brackets_curly()
    {
        assert_eq!(Some(1), match_brackets_curly("{}"));
        assert_eq!(Some(5), match_brackets_curly("{{{}}}"));
        assert_eq!(Some(6), match_brackets_curly("{{},{}}"));
        assert_eq!(Some(28), match_brackets_curly("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }

    #[test]
    fn test_score()
    {
        assert_eq!((3, 17), score("{{<a!>},{<a!>},{<a!>},{<ab>}}", 1));
        assert_eq!((9, 0), score("{{<!!>},{<!!>},{<!!>},{<!!>}}", 1));
        assert_eq!((9, 8), score("{{<ab>},{<ab>},{<ab>},{<ab>}}", 1));
        assert_eq!((1, 4), score("{<a>,<a>,<a>,<a>}", 1));
    }
}

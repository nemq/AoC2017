
pub fn first_puzzle() -> String
{
    let mut numbers: Vec<i32> = (0..256).collect();
    let lengths = vec![187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216];
    format!("{}", knot_sparse_hash(&mut numbers, &lengths))
}

pub fn second_puzzle() -> String
{
    let mut numbers: Vec<i32> = (0..256).collect();
    let lengths = lengths("187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216");
    format!("{}", knot_dense_hash(&mut numbers, &lengths))
}

fn reverse_range(numbers: &mut Vec<i32>, start: usize, count: usize)
{
    let mut reversed: Vec<_> = numbers.iter()
                                      .cloned()
                                      .cycle()
                                      .skip(start)
                                      .take(count)
                                      .collect();
    reversed.reverse();

    let num_it = (0..numbers.len()).cycle().skip(start).take(count);
    let rev_it = 0..reversed.len();
    for (num_idx, rev_idx) in num_it.zip(rev_it)
    {
        numbers[num_idx] = reversed[rev_idx];
    }
}

fn tie_knot_round(mut numbers: &mut Vec<i32>, lengths: &Vec<usize>, curr_pos: &mut usize, skip: &mut usize)
{
    for len in lengths.iter()
    {
        reverse_range(&mut numbers, *curr_pos, *len);
        *curr_pos = (*curr_pos + *len + *skip) % numbers.len();
        *skip += 1;
    }
}


fn tie_knot(numbers: &mut Vec<i32>, lengths: &Vec<usize>)
{
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64
    {
        tie_knot_round(numbers, lengths, &mut pos, &mut skip);
    }
}

fn knot_sparse_hash(mut numbers: &mut Vec<i32>, lengths: &Vec<usize>) -> i32
{
    tie_knot_round(&mut numbers, &lengths, &mut 0, &mut 0);
    numbers.iter().take(2).fold(1, |acc, &el| acc * el)
}

fn knot_dense_hash(mut numbers: &mut Vec<i32>, lengths: &Vec<usize>) -> String
{
    tie_knot(&mut numbers, &lengths);
    let mut hash = String::new();
    for chunk in numbers.as_slice().chunks(16)
    {
        let val = chunk.iter().fold(0, |acc, el| acc ^ el);
        hash.push_str(&format!("{:02x}",val));
    }

    hash
}

pub fn knot_hash(key: &str) -> String
{
    let mut numbers: Vec<i32> = (0..256).collect();
    let lengths = lengths(key);
    knot_dense_hash(&mut numbers, &lengths)
}

fn lengths(input: &str) -> Vec<usize>
{
    let mut lengths: Vec<usize> = input.as_bytes().iter().map(|&a| a as usize).collect();
    lengths.extend([17, 31, 73, 47, 23].iter());
    lengths
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_reverse_range() 
    {
        let mut numbers = vec![0, 1, 2, 3, 4];
        reverse_range(&mut numbers, 0,3);
        assert_eq!(numbers, vec![2, 1, 0, 3, 4]);

        let mut numbers = vec![2, 1, 0, 3, 4];
        reverse_range(&mut numbers, 3, 4);
        assert_eq!(numbers, vec![4, 3, 0, 1, 2]);

        let mut numbers = vec![4, 3, 0, 1, 2];
        reverse_range(&mut numbers, 2, 1);
        assert_eq!(numbers, vec![4, 3, 0, 1, 2]);

        let mut numbers = vec![4, 3, 0, 1, 2];
        reverse_range(&mut numbers, 1, 5);
        assert_eq!(numbers, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn test_tie_knot() 
    {
        let mut numbers = vec![0, 1, 2, 3, 4];
        tie_knot_round(&mut numbers, &vec![3, 4, 1, 5], &mut 0, &mut 0);
        assert_eq!(numbers, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn test_knot_sparse_hash()
    {
        let mut numbers = vec![0, 1, 2, 3, 4];
        let lengths = vec![3, 4, 1, 5];
        assert_eq!(knot_sparse_hash(&mut numbers, &lengths), 12);
    }

    #[test]
    fn test_lengths()
    {
        assert_eq!(lengths("1,2,3"), vec![49,44,50,44,51,17,31,73,47,23]);
    }

    #[test]
    fn test_knot_dense_hash()
    {
        let mut numbers: Vec<_> = (0..256).collect();
        let lengths = lengths("");
        assert_eq!(knot_dense_hash(&mut numbers, &lengths), String::from("a2582a3a0e66e6e86e3812dcb672a272"));
        assert_eq!(knot_hash(""), String::from("a2582a3a0e66e6e86e3812dcb672a272"));
    }
}
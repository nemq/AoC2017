use std::ops::Index;
use std::ops::IndexMut;
use std::error::Error;
use simple_error::SimpleError;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

macro_rules! pattern 
{
    ( $($( $x:expr ),*);* ) => {
        {
            let mut pattern = Pattern{ size: 0, data: Vec::new()};
            $(
                #[allow(unused_mut)]
                let mut row = Vec::new();
                $(
                    row.push($x);
                )*
                pattern.size = row.len();
                pattern.data.extend(row.iter());

            )*
            pattern
        }
    };
}

pub fn first_puzzle() -> String {
    let init_patt = pattern!['.', '#', '.';
                            '.', '.', '#';
                            '#', '#', '#'];
    

    let rules = Rule::from_file("enchancment.txt").unwrap();
    let mut curr_pattern = init_patt;
    for _ in 0 .. 5 {
        curr_pattern = Pattern::from_iter(curr_pattern.iter(), &rules);
    }

    format!("{}", curr_pattern.count_ones())
}

pub fn second_puzzle() -> String {
    let init_patt = pattern!['.', '#', '.';
                            '.', '.', '#';
                            '#', '#', '#'];
    
    let rules = Rule::from_file("enchancment.txt").unwrap();
    let mut curr_pattern = init_patt;
    for _ in 0 .. 18 {
        curr_pattern = Pattern::from_iter(curr_pattern.iter(), &rules);
    }

    format!("{}", curr_pattern.count_ones())
}


#[derive(Clone, PartialEq, Debug)]
struct Pattern {
    data: Vec<char>,
    size: usize
}

impl Pattern {
    fn zeros(size: usize) -> Pattern {
        let data = vec!['.'; size * size];
        Pattern {data, size}
    }

    fn vertical_flip(&self) -> Pattern {
        let size = self.size;
        let mut data = Vec::new();

        for r in (0 .. size).rev() {
            let iter = self.data.iter().skip(r * size).take(size);
            data.extend(iter);
        }
        Pattern{size, data}
    }

    fn horizontal_flip(&self) -> Pattern {
        let size = self.size;
        let mut data = Vec::new();
        for r in (0 .. size).rev() {
            let iter = self.data.iter().rev().skip(r * size).take(size);
            data.extend(iter);
        }

        Pattern{size, data}
    }

    fn rotate_left(&self) -> Pattern {
        let size = self.size;
        assert!(size == 2 || size == 3);
        let mut rotated = Pattern::zeros(size);

        for i in 0 .. size {
            rotated[(i ,0)] = self[(0, size -1 -i)];
            rotated[(size -1, i)] = self[(i, 0)];
            rotated[(size -1 - i, size -1)] = self[(size -1, i)];
            rotated[(0, size -1 - i)] = self[(size -1 -i, size -1)];
        }
        if size == 3 {
            let mid = 1;
            rotated[(mid, mid)] = self[(mid, mid)];
        }

        rotated
    }

    fn transformations(&self) -> Vec<Pattern> {
        let mut trans = Vec::new();
        trans.push(self.clone());
        let rot90 = self.rotate_left();
        let rot180 = rot90.rotate_left();
        let rot270 = rot180.rotate_left();
        trans.push(rot90);
        trans.push(rot180);
        trans.push(rot270);
        let vert = self.vertical_flip();
        let vert_rot90 = vert.rotate_left();
        let vert_rot180 = vert_rot90.rotate_left();
        let vert_rot270 = vert_rot180.rotate_left();
        trans.push(vert);
        trans.push(vert_rot90);
        trans.push(vert_rot180);
        trans.push(vert_rot270);
        let hori = self.horizontal_flip();
        let hori_rot90 = hori.rotate_left();
        let hori_rot180 = hori_rot90.rotate_left();
        let hori_rot270 = hori_rot180.rotate_left();
        trans.push(hori);
        trans.push(hori_rot90);
        trans.push(hori_rot180);
        trans.push(hori_rot270);
        trans
    }

    fn enchance(&self, rules: &Vec<Rule>) -> Pattern {
        for rule in rules {
            if rule.matches(self) {
                return rule.ench_pattern.clone()
            }
        }
        panic!("Failed to match pattern with given rules: {}", self);
    }

    fn next_size(size: usize) -> usize {
        if size % 2 == 0 {
            size / 2 * 3
        }
        else if size % 3 == 0 {
            size / 3 * 4
        }
        else {
            panic!("invalid fractal size: {}", size);
        }
    }

    fn step(&self) -> usize {
        if self.size % 2 == 0 {
            2
        }
        else if self.size % 3 == 0 {
            3
        }
        else {
            panic!("invalid pattern size: {}", self.size);
        }
    }

    fn iter<'a>(&'a self) -> PatternIterator<'a> {
        PatternIterator::new(self)
    }

    fn from_iter(mut iter: PatternIterator, rules: &Vec<Rule>) -> Pattern {
        let next_size = Pattern::next_size(iter.pattern.size);
        let mut next_pattern = Pattern::zeros(next_size);

        let mut row = 0;
        let mut col = 0;
        while let Some(pattern) = iter.next().map(|p| p.enchance(rules)) {
            for i in 0 .. pattern.size {
                let next_pattern_range = (row + i) * next_pattern.size + col .. (row + i) * next_pattern.size + col + pattern.size;
                next_pattern.data.splice(next_pattern_range, pattern.data.iter().skip(i * pattern.size).take(pattern.size).cloned());
            }

            if row == next_pattern.size {
                panic!("too many patterns"); 
            }
            else if col < next_pattern.size - pattern.size {
                col += pattern.size;
            }
            else {
                row += pattern.size;
                col = 0;
            }
        }
      next_pattern
    }

    fn count_ones(&self) -> usize {
        self.data.iter().filter(|&&e| e == '#').count()
    }
}



impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0 .. self.size {
            for c in 0 .. self.size {
                write!(f, "{}", self.data[r * self.size + c])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Index<(usize, usize)> for Pattern {
    type Output = char;

    fn index(&self, (row, col): (usize, usize)) -> &char { 
       &self.data[ row * self.size + col] 
    }
}

impl IndexMut<(usize, usize)> for Pattern {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut char { 
       &mut self.data[ row * self.size + col] 
    }
}

struct Rule {
    match_pattern: Pattern,
    ench_pattern: Pattern
}

impl Rule {
    fn parse(line: &str) -> Result<Rule, Box<Error>> {
        let mut iter  = line.split("=>").map(|token| token.trim());
        let match_pattern_token = iter.next().ok_or(SimpleError::new(&format!("missing input pattern ({})", line)))?;
        let match_pattern = Rule::parse_pattern(match_pattern_token)?;
        let ench_pattern_token = iter.next().ok_or(SimpleError::new(&format!("missing output pattern ({})", line)))?;
        let ench_pattern = Rule::parse_pattern(ench_pattern_token)?;

        Ok(Rule {match_pattern, ench_pattern})
    }

    fn parse_pattern(pattern: &str) -> Result<Pattern, Box<Error>> {
       let size  = pattern.matches('/').count() + 1;
       let mut mat = Pattern::zeros(size);
       let mut iter = pattern.split('/').map(|token| token.trim());
       for r in 0 .. size {
           let row_token = iter.next().ok_or(SimpleError::new(&format!("missing row {} ({})", r, pattern)))?;
           let range = (r * size) .. (r + 1) * size;
           mat.data.splice(range, row_token.chars()
                                           .map(|ch| match ch {
                                                   '.' | '#' => ch,
                                                   _ => panic!("invalid char: {}", ch)
                                           })).collect::<Vec<_>>();
       }

       Ok(mat)
    }

    fn matches(&self, pattern: &Pattern) -> bool {
        let trans = pattern.transformations();
        for patt in trans.iter() {
            if *patt == self.match_pattern {
                return true;
            }
        }

        false
    }

    fn from_file(path: &str) -> Result<Vec<Rule>, Box<Error>> {
        let mut f = File::open(path)?;
        let mut buff = String::new();
        f.read_to_string(&mut buff)?;
        let mut v = Vec::new();
        for l in buff.split('\n').filter(|l| !l.is_empty()) {
            let r = Rule::parse(l)?;
            v.push(r);
        }

        Ok(v)
    }
}

struct PatternIterator<'a> {
    row: usize,
    col: usize,
    pattern: &'a Pattern
}

impl<'a> PatternIterator<'a> {
    fn new(pattern: &'a Pattern) -> PatternIterator {
        PatternIterator {row: 0, col: 0, pattern}
    }

    fn pattern(&mut self) -> Pattern {
       let mut data = Vec::new(); 
       let step = self.pattern.step();
       for r in self.row .. self.row + step {
           data.extend(self.pattern.data.iter()
                                   .skip(r * self.pattern.size + self.col)
                                   .take(step));
       }

       Pattern {size: step, data}
    }
}

impl<'a> Iterator for PatternIterator<'a> {
    type Item = Pattern;

    fn next(&mut self) -> Option<Pattern> {
        if self.row == self.pattern.size  {
           return None
        }

        let pattern = self.pattern();
        if self.col < self.pattern.size - self.pattern.step() {
            self.col += self.pattern.step();
        }
        else {
            self.row += self.pattern.step();
            self.col = 0;
        }

        Some(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

  

    #[test]
    fn flips() {
        let pattern = pattern!['.', '#', '.';
                               '.', '.', '#';
                               '#',  '#',  '#'];
                                

        let vert = pattern.vertical_flip();
        assert_eq!(vert, pattern! ['#', '#', '#';
                                  '.', '.', '#';
                                  '.', '#', '.']);

        let hor = pattern.horizontal_flip();
        assert_eq!(hor, pattern! ['.', '#', '.';
                                 '#', '.', '.';
                                 '#', '#', '#']);
    }

    #[test] 
    fn rotations() {
        let pattern = pattern!['.', '#', '.';
                          '.', '.', '#';
                          '#', '#', '#'];

        let rot90 = pattern.rotate_left();
        assert_eq!(rot90, pattern!  ['.', '#', '#';
                                    '#', '.', '#';
                                    '.', '.', '#']);
        let rot180 = rot90.rotate_left();
        assert_eq!(rot180, pattern!  ['#', '#', '#';
                                     '#', '.', '.';
                                     '.', '#', '.']);
        let rot270 = rot180.rotate_left();
        assert_eq!(rot270, pattern!  ['#', '.', '.';
                                     '#', '.', '#';
                                     '#', '#', '.']);
    }

    #[test]
    fn rule_matches() {
        let rule = Rule::parse(".#./..#/### => #..#/..../..../#..#").unwrap();

        let pattern = pattern!['.', '#', '.';
                          '.', '.', '#';
                          '#', '#', '#'];
        let a = pattern!['.', '#', '.';
                          '#', '.', '.';
                          '#', '#', '#'];
        let b = pattern!['#', '.', '.';
                          '#', '.', '#';
                          '#', '#', '.'];
        let c = pattern!['#', '#', '#';
                          '.', '.', '#';
                          '.', '#', '.'];

        assert!(rule.matches(&pattern));
        assert!(rule.matches(&a));
        assert!(rule.matches(&b));
        assert!(rule.matches(&c));
    }
}

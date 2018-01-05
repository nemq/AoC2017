use std::collections::HashMap;
use std::io::prelude::*; 
use std::fs::File; 

pub fn first_puzzle() -> String {
    let mut diag = RoutingDiagram::from_file("routing.txt");
    while diag.next_move() {}
    format!("{}", diag.letters.iter().collect::<String>())
}

pub fn second_puzzle() -> String {
    let mut diag = RoutingDiagram::from_file("routing.txt");
    while diag.next_move() {}
    format!("{}", diag.steps + 1)
}

#[derive(PartialEq)]
enum Field { VertLine, HorizLine, Cross, Letter(char), Empty}
#[derive(Clone)]
enum Direction {Top, Right, Bottom, Left}

#[derive(Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {x, y}
    }
}

type Dim = Pos;

struct RoutingDiagram {
    diagram: HashMap<(usize, usize), char>,
    dims: Dim,
    pos: Pos,
    dir: Direction,
    letters: Vec<char>,
    steps: usize
}

impl RoutingDiagram {
    fn new() -> RoutingDiagram {
        RoutingDiagram{ 
            diagram: HashMap::new(),
            dims: Dim::new(0, 0),
            pos: Pos::new(0, 0),
            dir: Direction::Bottom,
            letters: Vec::new(),
            steps: 0}
    }

    fn next_pos(&self, pos: &Pos, dir: &Direction) -> Option<Pos> {
       let mut next_pos = pos.clone();
       match *dir {
           Direction::Top => {
               if pos.y == 0 {
                   None
               }
               else {
                  next_pos.y -= 1;
                  Some(next_pos)
               }
           },
           Direction::Right => {
               if pos.x == self.dims.x {
                   None
               } 
               else {
                   next_pos.x += 1;
                   Some(next_pos)
               }
           }
           Direction::Bottom => {
               if pos.y == self.dims.y {
                   None
               } 
               else {
                   next_pos.y += 1;
                   Some(next_pos)
               }
           }, 
           Direction::Left => {
               if pos.x == 0 {
                   None
               }
               else {
                  next_pos.x -= 1;
                  Some(next_pos)
               }
           }
       } 
    }

    fn from_str(lines: &str) -> RoutingDiagram {
        let mut diagram = RoutingDiagram::new();
        let mut height  = 0;
        let mut width = 0;
        for (y, line) in lines.split('\n').filter(|l| !l.is_empty()).enumerate() {
            width = usize::max(width, diagram.parse_row(y, line));
            height += 1;
        }

        diagram.dims = Dim::new(width, height); 
        diagram.pos = diagram.find_start_pos();
        diagram
    }

    fn from_file(path: &str) -> RoutingDiagram {
        let mut file = File::open(path).expect("Failed to open routing file.");
        let mut buff = String::new();
        if let Ok(_) = file.read_to_string(&mut buff) {
            return RoutingDiagram::from_str(&buff)
        }

        panic!("Failed to read routing file.");
    }

    fn parse_row(&mut self, y: usize, line: &str) -> usize {
        let mut max_x = 0;
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '|' | '-' | '+'  => {
                    max_x = x;
                    self.diagram.insert((x, y), ch);},
                l if l.is_alphabetic() => {
                    max_x = x;
                    self.diagram.insert((x, y), ch);},
                _ => {}
            }
        }
        max_x
    }

    fn move_dir(&mut self, dir: Direction) -> bool {
        let next_pos = self.next_pos(&self.pos, &dir);
        if next_pos.is_none() {
            return false;
        }
        let next_pos = next_pos.unwrap();

        let next_field = self.get(&next_pos);
        if next_field == Field::Empty {
            return false;
        }

        match (&dir, next_field) {
            (&Direction::Top, Field::Cross) |
            (&Direction::Top, Field::VertLine) |
            (&Direction::Bottom, Field::Cross) |
            (&Direction::Bottom, Field::VertLine) => {
                self.pos = next_pos;
                self.dir = dir;
                self.steps += 1;
                true
            },
            (&Direction::Top, Field::Letter(l)) |
            (&Direction::Bottom, Field::Letter(l)) => {
                self.letters.push(l);
                self.pos = next_pos;
                self.dir = dir;
                self.steps += 1;
                true
            },
            (&Direction::Top, Field::HorizLine) |
            (&Direction::Bottom, Field::HorizLine) => {
                let next_pos = self.next_pos(&next_pos, &dir);
                if next_pos.is_none() {
                    return false
                }
                let next_pos = next_pos.unwrap();
                let next_field = self.get(&next_pos);
                if next_field == Field::Empty {
                    return false;
                }

                match next_field {
                    Field::Cross | Field::VertLine => {
                        self.pos = next_pos;
                        self.dir = dir;
                        self.steps += 2;
                        true
                    }
                    Field::Letter(l) => {
                        self.letters.push(l);
                        self.pos = next_pos;
                        self.dir = dir;
                        self.steps += 2;
                        true
                    }, 
                    _ => false
                }
            },
            (&Direction::Left, Field::Cross) |
            (&Direction::Left, Field::HorizLine) |
            (&Direction::Right, Field::Cross) |
            (&Direction::Right, Field::HorizLine) => {
                self.pos = next_pos;
                self.dir = dir;
                self.steps += 1;
                true
            },
            (&Direction::Left, Field::Letter(l)) |
            (&Direction::Right, Field::Letter(l)) => {
                self.letters.push(l);
                self.pos = next_pos;
                self.dir = dir;
                self.steps += 1;
                true
            },
            (&Direction::Left, Field::VertLine) |
            (&Direction::Right, Field::VertLine) => {
                let next_pos = self.next_pos(&next_pos, &dir);
                if next_pos.is_none() {
                    return false
                }
                let next_pos = next_pos.unwrap();
                let next_field = self.get(&next_pos);
                if next_field == Field::Empty {
                    return false;
                }

                match next_field {
                    Field::Cross | Field::HorizLine => {
                        self.pos = next_pos;
                        self.dir = dir;
                        self.steps += 2;
                        true
                    }
                    Field::Letter(l) => {
                        self.letters.push(l);
                        self.pos = next_pos;
                        self.dir = dir;
                        self.steps += 2;
                        true
                    }, 
                    _ => false
                }
            },
            _ => false
        }
    }

    fn get(&self, pos: &Pos) -> Field {
        if let Some(&c) = self.diagram.get(&(pos.x, pos.y)) {
            match c {
                '|' => Field::VertLine,
                '-' => Field::HorizLine,
                '+' => Field::Cross,
                 l if l.is_alphabetic() => Field::Letter(l),
                _ => panic!("Unrecognized field character: {}", c)
            }
        }
        else {
            Field::Empty
        }
    }

    fn find_start_pos(&self) -> Pos {
        for x in 0 .. self.dims.x {
            let pos = Pos::new(x, 0);
            match self.get(&pos) {
                Field::VertLine => return Pos::new(x, 0),
                _ => {}
            }
        }

        panic!("Failed to find starting position.")
    }

    fn next_move(&mut self) -> bool {
        match self.get(&self.pos) {
            Field::Cross => self.cross_move(),
            Field::Empty => { panic!("Current pos should never be empty.")},
            _ => {
                let dir = self.dir.clone();
                self.move_dir(dir) 
            }
        }
    }

    fn turn_right(&self, dir: &Direction) -> Direction {
        match *dir {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom, 
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top
        }
    }

    fn turn_left(&self, dir: &Direction) -> Direction {
        match *dir {
            Direction::Top => Direction::Left,
            Direction::Right => Direction::Top, 
            Direction::Bottom => Direction::Right,
            Direction::Left => Direction::Bottom
        }
    }

    fn cross_move(&mut self) -> bool {
        let dir = self.dir.clone();
        if self.move_dir(dir) {
            return true
        }

        let dir = self.turn_left(&self.dir);
        if self.move_dir(dir) {
            return true
        }

        let dir = self.turn_right(&self.dir);
        if self.move_dir(dir) {
            return true
        }

        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_puzzle() {
        let mut diag = RoutingDiagram::from_str("
            |          
            |  +--+    
            A  |  C    
        F---|----E|--+ 
            |  |  |  D 
            +B-+  +--+ 
            ");

        println!("dims: {}x{}", diag.dims.x, diag.dims.y);
        println!("start: {}x{}", diag.pos.x, diag.pos.y);
        println!("{}", "=".repeat(20));
        while diag.next_move() {
            println!("pos: ({},{})", diag.pos.x, diag.pos.y);
            match diag.dir {
                Direction::Top => println!("top"),
                Direction::Right => println!("right"),
                Direction::Bottom => println!("bottom"),
                Direction::Left => println!("left"),
            }
            match diag.get(&diag.pos) {
                Field::VertLine => println!("vert"),
                Field::HorizLine => println!("horiz"),
                Field::Cross => println!("cross"),
                Field::Letter(l) => println!("letter({})", l),
                Field::Empty => println!("empty")
            }
            println!("{}", "=".repeat(20));
        }
        println!("letters: {:?}", diag.letters);
        assert_eq!(diag.letters, vec!['A','B','C','D','E','F']);
    }

    #[test]
    fn test_second_puzzle() {
        let mut diag = RoutingDiagram::from_str("
            |          
            |  +--+    
            A  |  C    
        F---|----E|--+ 
            |  |  |  D 
            +B-+  +--+ 
            ");
        while diag.next_move() {}

        assert_eq!(diag.steps + 1, 38);
    }
}
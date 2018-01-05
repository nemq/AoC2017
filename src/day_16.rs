use std::io::prelude::*; 
use std::io::BufReader; 
use std::fs::File; 
use std::collections::HashSet;

pub fn first_puzzle() -> String {
    let mut hall = DanceHall::new();
    let moves = DanceHall::read_moves("dance.txt");
    hall.performe_dance(&moves);
    hall.to_string()
}

pub fn second_puzzle() -> String {
    let mut hall = DanceHall::new();
    let moves = DanceHall::read_moves("dance.txt");
    let miliard = 1000000000;
    if let Some(cycle) = DanceHall::find_cycle(&moves, miliard) {
        let remainder = miliard % cycle;
        println!("cycle: {}\tremainder: {}",cycle, remainder);
        for _ in 0 .. remainder {
            hall.performe_dance(&moves);
        }
    }
    else {
        panic!("No cycle found!");
    }
    hall.to_string()
}

#[derive(Clone)]
#[derive(PartialEq)]
enum Program {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}

fn char_to_prog(c: char) -> Program {
    match c {
        'a' => Program::A,
        'b' => Program::B,
        'c' => Program::C,
        'd' => Program::D,
        'e' => Program::E,
        'f' => Program::F,
        'g' => Program::G,
        'h' => Program::H,
        'i' => Program::I,
        'j' => Program::J,
        'k' => Program::K,
        'l' => Program::L,
        'm' => Program::M,
        'n' => Program::N,
        'o' => Program::O,
        'p' => Program::P,
        _   => panic!("Invalid char: {}", c) 
    }
}
fn prog_to_char(prog: &Program) -> char {
    match *prog {
        Program::A => 'a',
        Program::B => 'b',
        Program::C => 'c',
        Program::D => 'd',
        Program::E => 'e',
        Program::F => 'f',
        Program::G => 'g',
        Program::H => 'h',
        Program::I => 'i',
        Program::J => 'j',
        Program::K => 'k',
        Program::L => 'l',
        Program::M => 'm',
        Program::N => 'n',
        Program::O => 'o',
        Program::P => 'p',
    }
}

enum DanceMove {Spin(usize), Exchange(usize, usize), Partner(Program, Program) }

struct DanceHall {
    dancers: Vec<Program>
}

impl DanceHall {
    fn new() -> DanceHall {
        let dancers = vec![Program::A, Program::B, Program::C, Program::D,
                           Program::E, Program::F, Program::G, Program::H,
                           Program::I, Program::J, Program::K, Program::L,
                           Program::M, Program::N, Program::O, Program::P];
        DanceHall{dancers}
    }

    fn parse(token: &str) -> DanceMove {
        match token.chars().nth(0) {
            Some('s') => {
                let count = token.chars()
                                 .skip(1)
                                 .collect::<String>()
                                 .parse::<usize>()
                                 .expect(&format!("Failed to parse count: {}",token));

                return DanceMove::Spin(count as usize)
            },
            Some('x') => {
                let first_idx = token.chars()
                                     .skip(1)
                                     .take_while(|c| *c != '/')
                                     .collect::<String>()
                                     .parse::<usize>()
                                     .expect(&format!("Failed to parse first_idx: {}", token));

                let second_idx = token.chars()
                                      .skip_while(|c| *c != '/')
                                      .skip(1)
                                      .collect::<String>()
                                      .parse::<usize>()
                                      .expect(&format!("Failed to parse second_idx: {}", token));

                return DanceMove::Exchange(first_idx as usize, second_idx as usize)
            },
            Some('p') => {
                let first_prog = char_to_prog(token.chars()
                                                   .nth(1)
                                                   .expect(&format!("Missing first_prog: {}", token)));
                let second_prog = char_to_prog(token.chars()
                                                    .nth(3)
                                                    .expect(&format!("Missing second_prog: {}", token)));

                return DanceMove::Partner(first_prog, second_prog)
            },
            _ => panic!("Invalid token: {}", token)
        }
    }

    fn perform_move(self: &mut Self, dance_move: &DanceMove) {
        match dance_move {
            &DanceMove::Spin(count) => self.spin(count),
            &DanceMove::Exchange(first_idx, second_idx) => self.exchange(first_idx, second_idx),
            &DanceMove::Partner(ref first_prog, ref second_prog) => self.partner(first_prog, second_prog)
        }
    }

    fn performe_dance(self: &mut Self, dance: &Vec<DanceMove>) {
        for m in dance.iter() {
            self.perform_move(m);
        }
    }

    fn spin(self: &mut Self, count: usize) {
        let len = self.dancers.len();
        let skip = len - count;
        self.dancers = self.dancers.iter()
                                   .cycle()
                                   .cloned()
                                   .skip(skip)
                                   .take(len)
                                   .collect();
    }

    fn exchange(self: &mut Self, first_idx: usize, second_idx: usize) {
        self.dancers.swap(first_idx, second_idx);
    }

    fn partner(self: &mut Self, first_prog: &Program, second_prog: &Program) {
        let first_idx = self.dancers.iter()
                                    .position(|prog| prog == first_prog)
                                    .expect(&format!("Failed to find first_prog: {}", prog_to_char(&first_prog)));

        let second_idx = self.dancers.iter()
                                     .position(|prog| prog == second_prog)
                                     .expect(&format!("Failed to find second_prog: {}", prog_to_char(&second_prog)));
        self.dancers.swap(first_idx, second_idx);
    }

    fn to_string(self: &Self) -> String {
       let mut hall = String::new();
        for prog in self.dancers.iter() {
            hall.push(prog_to_char(prog));
        }
       hall 
    }

    fn read_moves(path: &str) -> Vec<DanceMove> {
        let mut moves = Vec::new();
        let file = File::open(path).expect("Failed to open dance file"); 
        let reader = BufReader::new(file); 
        for line in reader.lines().filter_map(|res| res.ok()) {
            for token in line.split(',').map(|t| t.trim()) {
                moves.push(DanceHall::parse(token));
            }
        }
        moves
    }

    fn find_cycle(dance: &Vec<DanceMove>, max_repeat: usize) -> Option<usize> {
        let mut hall = DanceHall::new();
        let mut states = HashSet::new();
        for i in 0 .. max_repeat {
            let state = hall.to_string();
            if states.contains(&state) {
                return Some(i)
            }
            states.insert(state);
            hall.performe_dance(dance);
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(match DanceHall::parse("s1") {
            DanceMove::Spin(1) => true,
            _ => false
        });
        assert!(match DanceHall::parse("x3/4") {
            DanceMove::Exchange(3,4) => true,
            _ => false
        });
        assert!(match DanceHall::parse("pe/b") {
            DanceMove::Partner(Program::E, Program::B) => true,
            _ => false
        });
    }

    #[test]
    fn test_dance() {
        let mut hall = DanceHall::new();
        println!("init: {}", hall.to_string());
        hall.perform_move(&DanceHall::parse("s1"));
        println!("spin: {}", hall.to_string());
        hall.perform_move(&DanceHall::parse("x3/4"));
        println!("exchane: {}", hall.to_string());
        hall.perform_move(&DanceHall::parse("pe/b"));
        println!("partner: {}", hall.to_string());
        assert_eq!(&hall.to_string(), "paedcbfghijklmno");
    }
}
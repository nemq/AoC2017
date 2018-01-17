use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;

pub fn first_puzzle() -> String {
    let mut tape = Tape::new();
        tape.insert('A', Box::new(|tape: &mut Tape | -> char {
            if !tape.get() {
                tape.set(true);
                tape.move_right();
                'B'
            }
            else {
                tape.set(false);
                tape.move_left();
                'C'
            }
        }));

        tape.insert('B', Box::new(|tape: &mut Tape | -> char {
            if !tape.get() {
                tape.set(true);
                tape.move_left();
                'A'
            }
            else {
                tape.set(true);
                tape.move_right();
                'D'
            }
        }));

        tape.insert('C', Box::new(|tape: &mut Tape | -> char {
            if !tape.get() {
                tape.set(false);
                tape.move_left();
                'B'
            }
            else {
                tape.set(false);
                tape.move_left();
                'E'
            }
        }));

        tape.insert('D', Box::new(|tape: &mut Tape | -> char {
            if !tape.get() {
                tape.set(true);
                tape.move_right();
                'A'
            }
            else {
                tape.set(false);
                tape.move_right();
                'B'
            }
        }));
        tape.insert('E', Box::new(|tape: &mut Tape | -> char {
            if !tape.get() {
                tape.set(true);
                tape.move_left();
                'F'
            }
            else {
                tape.set(true);
                tape.move_left();
                'C'
            }
        }));
        tape.insert('F', Box::new(|tape: &mut Tape | -> char {
            if !tape.get() {
                tape.set(true);
                tape.move_right();
                'D'
            }
            else {
                tape.set(true);
                tape.move_right();
                'A'
            }
        }));

        tape.select('A');
        for _ in 0 .. 12481997 {
            tape.execute();
        }

    format!("{}", tape.checksum()) 
}

pub fn second_puzzle() -> String {
    format!("") 
}



type StateId = char;
type State = Box<Fn(&mut Tape)-> StateId>;

struct Tape {
    data: HashSet<i64>,
    pos: i64,
    cur: Option<StateId>,
    states: HashMap<StateId, State>
}

impl Tape {
    fn new() -> Tape {
        Tape {
            data: HashSet::new(),
            pos: 0, 
            cur: None,
            states: HashMap::new()}
    }

    fn get(&self) -> bool {
        self.data.contains(&self.pos)
    }

    fn set(&mut self, value: bool) {
        if value {
            self.data.insert(self.pos);
        }
        else {
            self.data.remove(&self.pos);
        }
    }

    fn move_left(&mut self) {
        self.pos -=1;
    }

    fn move_right(&mut self) {
        self.pos += 1;
    }

    fn select(&mut self, id: StateId) -> bool {
        if self.states.contains_key(&id) {
            self.cur = Some(id);
            true
        }
        else {
            false
        }
    }

    fn insert(&mut self, id: StateId, state: State) -> Option<State> {
        self.states.insert(id, state)
    }

    fn remove(&mut self, id: StateId) -> Option<State> {
        self.states.remove(&id)
    }

    fn execute(&mut self) {
        if let Some(id) = self.cur {
            if let Some(state) = self.states.remove(&id) {
                let next = state(self);
                self.states.insert(id, state);
                self.select(next);
            }
            else {
                panic!("invalid state selected: {}", id);
            }
        }
        else {
            panic!("no state selected");
        }
    }

    fn checksum(&mut self) -> u64 {
        self.data.len() as u64
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_opt = self.data.iter().min();
        let max_opt = self.data.iter().max();
        match (min_opt, max_opt) {
            (Some(&min), Some(&max)) => {
                let width = i64::max(min.abs(), max);
                let off = width / 2 + 1;
                for i in (0 .. width).map(|i| i - off) {
                    match self.data.get(&i) {
                        Some(_) => write!(f, "1 ")?,
                        _ => write!(f, "0 ")?,
                    }
                }
            },
            _ => write!(f, "0")?
        }
        write!(f, "")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle() {
        let mut tape = Tape::new();
        tape.insert('A', Box::new(|tape: &mut Tape | -> char {
            if tape.get() {
                tape.set(false);
                tape.move_left();
            }
            else {
                tape.set(true);
                tape.move_right();
            }
            'B'
        }));

        tape.insert('B', Box::new(|tape: &mut Tape | -> char {
            if tape.get() {
                tape.set(true);
                tape.move_right();
            }
            else {
                tape.set(true);
                tape.move_left();
            }
            'A'
        }));
        tape.select('A');

        for _ in 0 .. 6 {
            println!("{}", tape);
            tape.execute();
        }
        println!("{}", tape);
        println!("checksum {}", tape.checksum());
    }
}
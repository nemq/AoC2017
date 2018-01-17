use std::io::prelude::*; 
use std::io::BufReader; 
use std::fs::File; 
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn first_puzzle() -> String {
    let mut assembler = Assembler::new(0);
    assembler.load_program("assembly.txt");
    let mut freq = None;

    loop {
        if let Some(new_freq) = assembler.execute_next_instruction() {
            freq = Some(new_freq);
        }

        if assembler.terminated() {
            break;
        }

        if assembler.waiting_message() {
            let opp = assembler.instructions[assembler.ic as usize].split_whitespace()
                                                                   .skip(1)
                                                                   .next()
                                                                   .unwrap();
            if let Opperand::Reg(r) = parse(opp) {
                if *assembler.registers.get(&r).unwrap() > 0 {
                    break;
                }
            }

            assembler.messages.push_back(0);
        }
    }


    if let Some(f) = freq {
        format!("{}", f)
    }
    else {
        format!("Terminated without freq")
    }
}

pub fn second_puzzle() -> String {
    let mut first_prog = Assembler::new(0);
    first_prog.load_program("assembly.txt");
    let mut second_prog = Assembler::new(1);
    second_prog.load_program("assembly.txt");
    let mut snd_count = 0;

    while !(first_prog.waiting_message() && second_prog.waiting_message()) {
        if !first_prog.terminated() {
            if let Some(msg) = first_prog.execute_next_instruction() {
                second_prog.messages.push_back(msg);
            }
        }

        if !second_prog.terminated() {
            if let Some(msg) = second_prog.execute_next_instruction() {
                first_prog.messages.push_back(msg);
                snd_count +=1;
            }
        }

        if first_prog.terminated() && second_prog.terminated() {
            break;
        }

        if first_prog.terminated() && second_prog.waiting_message() {
            break;
        }

        if first_prog.waiting_message() && second_prog.terminated() {
            break;
        }
    }

    format!("{}", snd_count)
}


pub struct Assembler {
    pub registers: HashMap<char, i64>,
    pub instructions: Vec<String>,
    messages: VecDeque<i64>,
    pub ic: i64
}

enum Opperand {
    Reg(char), Const(i64)
}

fn parse(token: &str) -> Opperand {
    if token.len() == 1 && token.chars().next().unwrap().is_alphabetic() {
        return Opperand::Reg(token.chars().next().unwrap())
    }
    else {
        let c = token.parse().expect(&format!("Failed to parse constant opperand: {}", token));
        return Opperand::Const(c)
    }
}

impl Assembler {
    pub fn new(id: i64) -> Assembler {
        let mut registers = HashMap::new();
        for c in "abcdefghijklmnoprstuvwxyz".chars() {
            registers.insert(c, 0);
        }
        *registers.get_mut(&'p').unwrap() = id;
        Assembler {
            registers,
            instructions: Vec::new(), 
            messages: VecDeque::new(),
            ic: 0 }
    }

    #[allow(dead_code)]
    pub fn dump_regs(&self) -> String {
        let mut s = String::new();
        let mut keys = self.registers.keys().collect::<Vec<_>>();
        keys.sort();
        for k in keys.iter() {
            s.push_str(&format!("{}:{} |", k, self.registers[&k]));
        }
        s
    }

    pub fn load_program(&mut self, path: &str) {
        let file = File::open(path).expect("Failed to open assembly file"); 
        let reader = BufReader::new(file); 
        for line in reader.lines().filter_map(|res| res.ok()).filter(|l| !l.is_empty()) {
           self.instructions.push(String::from(line.trim())); 
        }
    }

    #[cfg(test)]
    pub fn load_program_str(&mut self, prog: &str) {
        for line in prog.split('\n').filter(|l| !l.is_empty()) {
            self.instructions.push(String::from(line.trim()));
        }
    }

    pub fn terminated(&self) -> bool {
        self.ic < 0 || self.ic >= self.instructions.len() as i64
    }

    fn waiting_message(&self) -> bool {
        if self.messages.len() > 0 {
            false
        }
        else {
            let op = (&self.instructions[self.ic as usize]).split_whitespace().next().unwrap();
            match op {
                "rcv" => true, 
                _     => false
            }
        }
    }

    fn snd(&mut self, instr: &str) -> i64 {
        let res = match instr.split_whitespace().nth(1) {
            Some(token) => {
                match parse(token) {
                    Opperand::Reg(r) => {
                        self.registers[&r]
                    },
                    Opperand::Const(c)  => {
                        c
                    }
                }
            }, 
            None => panic!("snd: Missing operand: {}", instr)
        };
        self.ic += 1;
        res
    } 

    fn rcv(&mut self, instr: &str) {
        match instr.split_whitespace().skip(1).next() {
            Some(token) => {
                match parse(token) {
                    Opperand::Reg(r) => {
                        if let Some(v) = self.messages.pop_front() {
                            *self.registers.get_mut(&r).unwrap() = v;        
                            self.ic += 1;
                        }
                    },
                    _ => {}
                }
            }, 
            None => panic!("rcv: Missing opperand: {}", instr)
        }
    } 

    fn arithmetic(&mut self, instr: &str) {
        let mut iter = instr.split_whitespace();
        let op = iter.next().unwrap();
        let opp_one = parse(iter.next().expect(&format!("arithmetic: Missing first opperand: {}", instr)));
        let opp_two = parse(iter.next().expect(&format!("arithmetic: Missing second opperand: {}", instr)));
        match (opp_one, opp_two) {
            (Opperand::Reg(target), Opperand::Reg(source)) => { 
                let val = self.registers[&source];
                match op {
                   "set" =>  *self.registers.get_mut(&target).unwrap() = val,
                   "add" =>  *self.registers.get_mut(&target).unwrap() += val,
                   "mul" =>  *self.registers.get_mut(&target).unwrap() *= val,
                   "mod" =>  *self.registers.get_mut(&target).unwrap() %= val,
                   "sub" =>  *self.registers.get_mut(&target).unwrap() -= val,
                    _ => {}
                }
            },
            (Opperand::Reg(target), Opperand::Const(val)) => {
                match op {
                   "set" =>  *self.registers.get_mut(&target).unwrap() = val,
                   "add" =>  *self.registers.get_mut(&target).unwrap() += val,
                   "mul" =>  *self.registers.get_mut(&target).unwrap() *= val,
                   "mod" =>  *self.registers.get_mut(&target).unwrap() %= val,
                   "sub" =>  *self.registers.get_mut(&target).unwrap() -= val,
                    _ => {}
                }
            },
            _ => panic!("set: Invalid opperands: {}", instr)
        }
        self.ic += 1;
    }

    fn jgz(&mut self, instr: &str) {
        let mut iter = instr.split_whitespace();
        iter.next();
        let opp_one = parse(iter.next().expect(&format!("jgz: Missing first opperand: {}", instr)));
        let opp_two = parse(iter.next().expect(&format!("jgz: Missing second opperand: {}", instr)));
        match opp_one {
            Opperand::Reg(r) if (self.registers[&r] > 0) => {
                match opp_two {
                    Opperand::Reg(r) => self.ic += self.registers[&r],
                    Opperand::Const(c) => self.ic += c
                }
            }, 
            Opperand::Const(c) if c > 0 => {
                match opp_two {
                    Opperand::Reg(r) => self.ic += self.registers[&r],
                    Opperand::Const(c) => self.ic += c
                }
            },
            _ => { self.ic += 1}
        }
    }

    fn jnz(&mut self, instr: &str) {
        let mut iter = instr.split_whitespace();
        iter.next();
        let opp_one = parse(iter.next().expect(&format!("jnz: Missing first opperand: {}", instr)));
        let opp_two = parse(iter.next().expect(&format!("jnz: Missing second opperand: {}", instr)));
        match opp_one {
            Opperand::Reg(r) if (self.registers[&r] != 0) => {
                match opp_two {
                    Opperand::Reg(r) => self.ic += self.registers[&r],
                    Opperand::Const(c) => self.ic += c
                }
            }, 
            Opperand::Const(c) if c != 0 => {
                match opp_two {
                    Opperand::Reg(r) => self.ic += self.registers[&r],
                    Opperand::Const(c) => self.ic += c
                }
            },
            _ => { self.ic += 1}
        }
    }
    pub fn execute_next_instruction(&mut self) -> Option<i64> {
        let instruction = self.instructions[self.ic as usize].clone();
        match instruction.split_whitespace().next() {
            Some("snd") => {
                Some(self.snd(&instruction))
                },
            Some("set") |
            Some("add") |
            Some("mul") | 
            Some("mod") |
            Some("sub") => {
                self.arithmetic(&instruction);
                None
                },
            Some("rcv") => {
                self.rcv(&instruction);
                None
                },
            Some("jgz") => {
                self.jgz(&instruction);
                None
                },
            Some("jnz") => {
                self.jnz(&instruction);
                None
            },
            _   => {
                panic!("Invalid instruciton: {}", instruction);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler() {
        let mut assembler = Assembler::new(0);
        let prog = "
            set a 1
            add a 2
            mul a a
            mod a 5
            snd a
            set a 0
            rcv a
            jgz a -1
            set a 1
            jgz a -2
        ";
        assembler.load_program_str(prog);
        assert_eq!(assembler.registers[&'a'], 0);
        assembler.execute_next_instruction();
        assert_eq!(assembler.registers[&'a'], 1);
        assembler.execute_next_instruction();
        assert_eq!(assembler.registers[&'a'], 3);
        assembler.execute_next_instruction();
        assert_eq!(assembler.registers[&'a'], 9);
        assembler.execute_next_instruction();
        assert_eq!(assembler.registers[&'a'], 4);
        assert_eq!(assembler.execute_next_instruction(), Some(4));
        assembler.execute_next_instruction();
        assert_eq!(assembler.registers[&'a'], 0);
        assert_eq!(assembler.execute_next_instruction(), None);
        assembler.execute_next_instruction();
        assert_eq!(assembler.ic, 6);
        assembler.execute_next_instruction();
        assert_eq!(assembler.registers[&'a'], 0);
        assembler.execute_next_instruction();
        assert_eq!(assembler.ic, 6);
        assembler.execute_next_instruction();
        assert_eq!(assembler.ic, 6);
        assembler.execute_next_instruction();
    }
}

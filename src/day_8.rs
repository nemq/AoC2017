use std::i32;
use std::cmp;
use std::collections::HashMap;
use std::io::prelude::*; 
use std::io::BufReader; 
use std::fs::File; 

pub fn first_puzzle() -> String
{
    let mut regs = Registers::new();
    let instrs = read_instructions("instructions.txt");
    for ins in instrs.iter()
    {
        ins.execute(&mut regs);
    }

    format!("{}", regs.max().unwrap())
}

pub fn second_puzzle() -> String
{
    let mut regs = Registers::new();
    let mut max = i32::MIN;
    let instrs = read_instructions("instructions.txt");
    for ins in instrs.iter()
    {
        ins.execute(&mut regs);
        if let Some(curr_max) = regs.max()
        {
            max = cmp::max(max, curr_max);
        }
    }

    format!("{}", max)
}

fn read_instructions(path: &str) -> Vec<Instruction>
{
    let mut instrs = Vec::new();
    let file = File::open(path).expect("Failed to open instruction file"); 
    let reader = BufReader::new(file); 
    for line in reader.lines().filter_map(|res| res.ok())
    {
        match Instruction::parse(&line)
        {
            Ok(ins) => instrs.push(ins),
            Err(msg) => println!("{}", msg)
        }
    }

    instrs
}

struct Registers
{
    registers: HashMap<String, i32>
}

impl Registers
{
    fn new() -> Registers
    {
        Registers{
            registers: HashMap::new()
        }
    }

    fn get(self: &mut Self, name: &str) -> &mut i32
    {
        self.registers.entry(String::from(name)).or_insert(0)
    }

    fn max(self: &Self) -> Option<i32>
    {
        self.registers.values().cloned().max()
    }
}

struct Instruction {
    target_reg: String,
    target_fun: Box<Fn(&mut i32)>,
    condition_reg: String,
    condition_fun: Box<Fn(&i32)->bool>,
}

impl Instruction
{
    fn new() -> Instruction
    {
        Instruction{
            target_reg: String::new(),
            target_fun: Box::new(|_| {}),
            condition_reg: String::new(),
            condition_fun: Box::new(|_| false),
        }
    }

    fn parse(line: &str) -> Result<Instruction, String>
    {
        let mut inst = Instruction::new();
        let mut op = String::new();
        let mut cond = String::new();

        for (idx, token) in line.split_whitespace().enumerate()
        {
            match (idx, token)
            {
                (0, _) => inst.target_reg = String::from(token),
                (1, "inc") | (1, "dec") =>  op = String::from(token),
                (2, _) if token.parse::<i32>().is_ok() => {
                    let arg = token.parse::<i32>().unwrap();
                    if op == "inc" {
                        inst.target_fun = Box::new(move |reg| *reg += arg);
                    }
                    else {
                        inst.target_fun = Box::new(move |reg| *reg -= arg);
                    }
                }
                (3, "if") => {},
                (4, _) => inst.condition_reg = String::from(token),
                (5, ">")  | (5, "<")  |
                (5, ">=") | (5, "<=") |
                (5, "==") | (5, "!=") => {
                    cond = String::from(token);
                },
                (6, _) if token.parse::<i32>().is_ok() => {
                  let arg = token.parse::<i32>().unwrap();
                  match cond.as_str() {
                      ">" => inst.condition_fun = Box::new(move |&reg| reg > arg),
                      "<" => inst.condition_fun = Box::new(move |&reg| reg < arg),
                      ">=" => inst.condition_fun = Box::new(move |&reg| reg >= arg),
                      "<=" => inst.condition_fun = Box::new(move |&reg| reg <= arg),
                      "==" => inst.condition_fun = Box::new(move |&reg| reg == arg),
                      "!=" => inst.condition_fun = Box::new(move |&reg| reg != arg),
                      _ => {}
                  };      
                },
                _ => return Err(format!("Error parsing {} token: {}", idx, token))
            };
        }

        Ok(inst)
    }

    fn condition_met(self: &Self, regs: &mut Registers) ->bool
    {
        (self.condition_fun)(regs.get(&self.condition_reg))
    }

    fn execute(self: &Self, regs: &mut Registers)
    {
        if self.condition_met(regs)
        {
            (self.target_fun)(regs.get(&self.target_reg))
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn first_puzzle() 
    {
        let mut regs = Registers::new();
        let mut instrs = Vec::new();
        instrs.push(Instruction::parse("b inc 5 if a > 1").unwrap());
        instrs.push(Instruction::parse("a inc 1 if b < 5").unwrap());
        instrs.push(Instruction::parse("c dec -10 if a >= 1").unwrap());
        instrs.push(Instruction::parse("c inc -20 if c == 10").unwrap());

        for inst in instrs.iter()
        {
            inst.execute(&mut regs);
        }

        assert_eq!(*regs.get("a"), 1);
        assert_eq!(*regs.get("b"), 0);
        assert_eq!(*regs.get("c"), -10);
        assert_eq!(regs.max(), Some(1));
    }

    #[test]
    fn second_puzzle() 
    {
        let mut regs = Registers::new();
        let mut instrs = Vec::new();
        instrs.push(Instruction::parse("b inc 5 if a > 1").unwrap());
        instrs.push(Instruction::parse("a inc 1 if b < 5").unwrap());
        instrs.push(Instruction::parse("c dec -10 if a >= 1").unwrap());
        instrs.push(Instruction::parse("c inc -20 if c == 10").unwrap());

        let mut max = i32::MIN;
        for inst in instrs.iter()
        {
            inst.execute(&mut regs);
            max = cmp::max(max, regs.max().unwrap());
        }

        assert_eq!(max, 10);
    }
}
use day_18::*;

pub fn first_puzzle() -> String {
    let mut assembler = Assembler::new(0);
    assembler.load_program("coprocessor.txt");
    let mut mul_counter = 0;
    while !assembler.terminated() {
        if assembler.instructions[assembler.ic as usize].starts_with("mul") {
            mul_counter += 1;
        }
        assembler.execute_next_instruction();
    }

    format!("{}", mul_counter)
}


pub fn second_puzzle() -> String {
    let mut b = 108100;
    let c = 125100;
    let mut h = 0;

    loop {
        if !is_prime(b){
            h += 1 
        }
        if b == c {
            break 
        }
        b += 17 
    }

    format!("{}", h)
}

fn is_prime(n: i64) -> bool {
    let sqrt = f64::sqrt(n as f64) as i64;
    for i in 2 .. sqrt {
        if n % i == 0 {
            return false
        }
    }

    true
}
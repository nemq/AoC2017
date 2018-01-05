
pub fn first_puzzle() -> String {
    let judge = Judge::new(Generator::a(679), Generator::b(771));
    format!("{}", judge.judge(40000000))
}

pub fn second_puzzle() -> String {
    let judge = Judge::new(Generator::a(679), Generator::b(771));
    format!("{}", judge.judge_cond(5000000))
}


struct Generator {
    factor: u64,
    previous: u64,
    mod_cond: u64,
}

impl Generator {
    fn new(factor: u64, start_val: u64, mod_cond: u64) -> Generator {
        Generator {factor, previous: start_val, mod_cond}
    }

    fn divisor() -> u64 {
        2147483647
    }

    fn a(start_val: u64) -> Generator {
        Generator::new(16807, start_val, 4)
    }

    fn b(start_val: u64) -> Generator {
        Generator::new(48271, start_val, 8)
    }

    fn generate(self: &mut Self) -> u64 {
        self.previous = (self.previous * self.factor) % Generator::divisor();
        self.previous
    }

    fn generate_cond(self: &mut Self) -> Option<u64> {
        self.generate();
        if self.previous % self.mod_cond == 0 {
            Some(self.previous)
        }
        else {
            None
        }
    }
}

struct Judge {
    a: Generator,
    b: Generator,
}

impl Judge {
    fn new(a: Generator, b: Generator) -> Judge {
        Judge {a, b}
    }

    fn equal(self: &Self, a: u64, b: u64) -> bool {
        (a & 0xFFFF) == (b & 0xFFFF)
    }

    fn judge(mut self: Self, attempts: u64) -> u64 {
        let mut matches = 0;
        for _ in 0..attempts {
            let a = self.a.generate();
            let b = self.b.generate();
            if self.equal(a, b) {
                matches += 1;
            }
        }
        matches
    }

    fn judge_cond(mut self: Self, attempts: u64) -> u64 {
        let mut matches = 0;
        for _ in  0 .. attempts {
           let a  = loop {
               if let Some(val) = self.a.generate_cond() {
                   break val;
               }
           };
           let b = loop {
               if let Some(val) = self.b.generate_cond() {
                   break val;
               }
           };

           if self.equal(a, b) {
               matches +=1;
           }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generators() {
        let mut a = Generator::a(65);
        let mut b = Generator::b(8921);
        assert_eq!(a.generate(), 1092455);
        assert_eq!(b.generate(), 430625591);
        assert_eq!(a.generate(), 1181022009);
        assert_eq!(b.generate(), 1233683848);
        assert_eq!(a.generate(), 245556042);
        assert_eq!(b.generate(), 1431495498);
    }

    #[test]
    fn test_judge() {
        let judge =Judge::new(Generator::a(65), Generator::b(8921));
        assert_eq!(judge.judge(5), 1);
        let judge =Judge::new(Generator::a(65), Generator::b(8921));
        assert_eq!(judge.judge(40000000), 588);
    }

    #[test]
    fn test_judge_cond() {
        let judge = Judge::new(Generator::a(65), Generator::b(8921));
        assert_eq!(judge.judge_cond(1056), 1);
    }
}
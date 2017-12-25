
pub fn first_puzzle() -> String {
    let mut judge = Judge::new(Generator::A(679), Generator::B(771));
    format!("{}", judge.judge(40000000))
}

pub fn second_puzzle() -> String {
    let mut judge = Judge::new(Generator::A(679), Generator::B(771));
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

    fn A(start_val: u64) -> Generator {
        Generator::new(16807, start_val, 4)
    }

    fn B(start_val: u64) -> Generator {
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
    A: Generator,
    B: Generator,
}

impl Judge {
    fn new(A: Generator, B: Generator) -> Judge {
        Judge {A, B}
    }

    fn equal(self: &Self, a: u64, b: u64) -> bool {
        (a & 0xFFFF) == (b & 0xFFFF)
    }

    fn judge(mut self: Self, attempts: u64) -> u64 {
        let mut matches = 0;
        for _ in 0..attempts {
            let a = self.A.generate();
            let b = self.B.generate();
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
               if let Some(val) = self.A.generate_cond() {
                   break val;
               }
           };
           let b = loop {
               if let Some(val) = self.B.generate_cond() {
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
        let mut A = Generator::A(65);
        let mut B = Generator::B(8921);
        assert_eq!(A.generate(), 1092455);
        assert_eq!(B.generate(), 430625591);
        assert_eq!(A.generate(), 1181022009);
        assert_eq!(B.generate(), 1233683848);
        assert_eq!(A.generate(), 245556042);
        assert_eq!(B.generate(), 1431495498);
    }

    #[test]
    fn test_judge() {
        let mut judge =Judge::new(Generator::A(65), Generator::B(8921));
        assert_eq!(judge.judge(5), 1);
        let mut judge =Judge::new(Generator::A(65), Generator::B(8921));
        assert_eq!(judge.judge(40000000), 588);
    }

    #[test]
    fn test_judge_cond() {
        let mut judge = Judge::new(Generator::A(65), Generator::B(8921));
        assert_eq!(judge.judge_cond(1056), 1);
    }
}
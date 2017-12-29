use std::fmt;
use std::collections::HashMap;

pub fn first_puzzle() -> String {

    let mut lock = SpinLock::new(324);
    for _ in 0 .. 2017 {
        lock.spin();
    }
    format!("{}", lock.buffer[lock.pos + 1])
}

pub fn second_puzzle() -> String {
    let fifty_milions = 50000000;
    let mut lock = SpinLockSim::new(324, fifty_milions + 1);
    for t in (0..(fifty_milions+1)).rev() {
            if lock.pos(t) == 1 {
                return format!("{}", SpinLockSim::val(t));
            }
    }

    panic!("Pos 1 never hit!");
}

struct SpinLock {
    step: usize,
    buffer: Vec<usize>,
    pos: usize
}

impl SpinLock {
    fn new(step: usize) -> SpinLock {
        let mut buffer = Vec::with_capacity(2018);
        buffer.push(0);
        SpinLock { step, buffer: buffer, pos: 0 }
    }



    fn next_pos(&self) -> usize {
        (self.pos + self.step) % self.buffer.len() + 1
    }

    fn nex_val(&self) -> usize {
        self.buffer.len()
    }

    fn spin(&mut self) {
        let val = self.nex_val();
        self.pos = self.next_pos();
        if self.pos < self.buffer.len() {
            self.buffer.insert(self.pos, val);
        }
        else if (self.pos == self.buffer.len()) {
            self.buffer.push(val);
        }
        else {
            panic!("Invalid pos: {:?}", self);
        }
    }
}

struct SpinLockSim {
    step: usize,
    cache_size: usize,
    cache: HashMap<usize, usize>
}

impl SpinLockSim {
    fn new(step: usize, cache_size: usize) -> SpinLockSim {
        let mut cache = HashMap::with_capacity(cache_size);
        cache.insert(0, 0);
        for t in 1 .. cache_size {
            let pos = (cache[&(t-1)] + step) % SpinLockSim::buffer_len(t-1) + 1;
            cache.insert(t, pos);
        }
        SpinLockSim {step, cache_size, cache}
    }

    fn buffer_len(time: usize) -> usize {
        1 + time
    }

    fn pos(&mut self, time: usize) -> usize {
        if time < self.cache_size {
            self.cache[&time]
        }
        else {
            panic!("Not implemented!");
        }
    }

    fn val(time: usize) -> usize {
        SpinLockSim::buffer_len(time) -1
    }
}

impl fmt::Debug for SpinLock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos: {}\t step: {}\n{:?}", self.pos, self.step, self.buffer) 
    }
}

use std::error::Error;
use std::fmt;
use std::io::prelude::*; 
use std::fs::File; 
use std::collections::HashMap;

pub fn first_puzzle() -> String {
    let system = ParticleSystem::from_file("particles.txt").unwrap();

    let mut min_acc_idx = 0;
    let mut min_acc = system.particles[min_acc_idx].acc.manhatan_norm();
    for (idx, p) in system.particles.iter().enumerate() {
        if p.acc.manhatan_norm() < min_acc {
            min_acc_idx = idx;
            min_acc = p.acc.manhatan_norm();
        }
    }

    format!("idx: {}\tacc: {}", min_acc_idx, min_acc)
}

pub fn second_puzzle() -> String {

    let mut system = ParticleSystem::from_file("particles.txt").unwrap();

    let mut particles_count = system.particles.len();
    let mut repeat_count = 0;

    while repeat_count < 10000 {
        system.update();
        system.remove_collisions();
        if system.particles.len() == particles_count {
            repeat_count += 1;
        }
        else {
            particles_count = system.particles.len();
            repeat_count = 0;
        }
    }
    format!("{}", particles_count)
}

#[derive(Debug)]
struct SimpleError {
   msg: String,
}

impl SimpleError {
    fn new(msg: &str) -> SimpleError {
        SimpleError{msg: String::from(msg)}
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleError: {}", self.msg)
    }
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        &self.msg
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Vec3 {
    x: i64,
    y: i64, 
    z: i64
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Vec3 {
        Vec3{x, y, z}
    }

    fn parse(s: &str) -> Result<Vec3, Box<Error>> {
        let mut iter = s.split(',').map(|t| t.trim());
        let x = iter.next()
                    .ok_or(SimpleError::new(&format!("x missing ({0})", s)))?
                    .parse::<i64>()?;
        let y = iter.next()
                    .ok_or(SimpleError::new(&format!("y missing ({0})", s)))?
                    .parse::<i64>()?;
        let z = iter.next()
                    .ok_or(SimpleError::new(&format!("z missing ({0})", s)))?
                    .parse::<i64>()?;

        Ok(Vec3::new(x, y, z))
    }

    fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn manhatan_norm(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Display for Vec3 {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Particle {
    pos: Vec3,
    vel: Vec3, 
    acc: Vec3,
}

impl Particle {
    fn new(pos: Vec3, vel: Vec3, acc: Vec3) -> Particle {
        Particle {pos, vel, acc}
    }

    fn parse(line: &str) -> Result<Particle, Box<Error>> {
        let mut iter = line.split(|ch| ch == '<' || ch == '>')
                           .map(|t| t.trim()).skip(1);
                           
        let pos_token = iter.next().ok_or(SimpleError::new(&format!("missing pos: {}", line)))?;
        let pos = Vec3::parse(pos_token)?;
        iter.next();

        let vel_token = iter.next().ok_or(SimpleError::new(&format!("missing pos: {}", line)))?;
        let vel = Vec3::parse(vel_token)?;
        iter.next();

        let acc_token = iter.next().ok_or(SimpleError::new(&format!("missing pos: {}", line)))?;
        let acc = Vec3::parse(acc_token)?;
        Ok(Particle::new(pos, vel, acc))
    }
}

impl fmt::Display for Particle {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[pos: {}\t vel: {}\tacc: {}]", self.pos, self.vel, self.acc)
    }
}


struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new() -> ParticleSystem {
        ParticleSystem { particles: Vec::new()}
    }

    fn from_str(s: &str) -> Result<ParticleSystem, Box<Error>> {
        let mut system = ParticleSystem::new();
        for l in s.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()) {
            let p = Particle::parse(l)?;
            system.particles.push(p)
        }
        Ok(system)
    }

    fn from_file(path: &str) -> Result<ParticleSystem, Box<Error>> {
        let mut file = File::open(path)?;
        let mut buff = String::new();
        file.read_to_string(&mut buff)?;
        ParticleSystem::from_str(&buff)
    }

    fn update(&mut self) {
        for p in self.particles.iter_mut() {
            p.vel.add(&p.acc);
            p.pos.add(&p.vel);
        }
    }

    fn remove_collisions(&mut self) {
        let mut uncollided = Vec::new();
        {
            let mut pos_map = HashMap::new();
            for p in self.particles.iter() {
                let entry = pos_map.entry(&p.pos).or_insert((p.clone(), 0));
                let count = &mut entry.1;
                *count += 1;
            }

            for (_, v) in pos_map.drain().filter(|&(_, (_, count))| count == 1) {
                let (p, _) = v;
                uncollided.push(p);
            }
        }
        self.particles = uncollided;
    }
}

impl fmt::Display for ParticleSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in self.particles.iter() {
            writeln!(f, "{}", p)?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_parse() {
        let s = "3,0,0";
        let v = Vec3::parse(s).unwrap();
        assert_eq!(v, Vec3::new(3,0,0));
        let s = "2,0,0";
        let v = Vec3::parse(s).unwrap();
        assert_eq!(v, Vec3::new(2,0,0));
        let s = "-1,0,0";
        let v = Vec3::parse(s).unwrap();
        assert_eq!(v, Vec3::new(-1,0,0));
    }

    #[test]
    fn particle_parse() {
        let l = "p=<3,0,0>, v=< 2,0,0>, a=<-1,0,0>";
        let p = Particle::parse(l).unwrap();
        assert_eq!(p, Particle::new(
            Vec3::new(3, 0, 0),
            Vec3::new(2, 0, 0),
            Vec3::new(-1, 0, 0)
        ));
    }
}
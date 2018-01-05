use std::io::prelude::*; 
use std::io::BufReader; 
use std::fs::File; 
use std::collections::HashMap;

pub fn first_puzzle() -> String
{
    let mut firewall = Firewall::from_file("firewall.txt");
    format!("{}", firewall.severity())
}

pub fn second_puzzle() -> String
{
    let mut firewall = Firewall::from_file("firewall.txt");
    format!("{}", firewall.delay())
}

enum Dir { Up, Down}
enum Caught {Yes(u32), No}

struct Layer
{
    range: u32,
    depth: u32,
    scanner_pos: u32,
    scanner_dir: Dir,
    period: u32
}

impl Layer
{
    fn new(depth: u32, range: u32) -> Layer
    {
        let period = 2 * (range - 1);
        Layer{depth,
              range,
              scanner_pos:0,
              scanner_dir: Dir::Down,
              period}
    }

    fn move_scanner(self: &mut Self)
    {
        match (self.scanner_pos, &self.scanner_dir)
        {
            (pos, &Dir::Down) if pos == self.range -1 => {
                self.scanner_dir = Dir::Up;
                self.scanner_pos -= 1;
            }
            (_, &Dir::Down)  =>  self.scanner_pos += 1,
            (pos, &Dir::Up) if pos == 0 => {
                self.scanner_dir = Dir::Down;
                self.scanner_pos += 1;
            }
            (_, &Dir::Up)  => self.scanner_pos -= 1,
        }
    }

    fn severity(self: &Self) -> u32
    {
        self.depth * self.range
    }

    fn caught(self: &Self) -> Caught
    {
        if self.scanner_pos == 0
        {
            Caught::Yes(self.severity())
        }
        else
        {
            Caught::No
        }
    }

    fn scanner_pos(self: &Self, time: u32) -> u32
    {
       let n = time % self.period;
       if n <= (self.period /2)
       {
           return n
       } 
       {
           return self.period - n
       }
    }

    fn parse(line: &str) -> Layer
    {
        let mut iter = line.split(':').map(|token| token.trim());
        let depth_token = iter.next().expect(&format!("Failed to read layer depth: {}", line));
        let depth = depth_token.parse().expect(&format!("Failed to parse layerdepth: {}", depth_token));
        let range_token = iter.next().expect(&format!("Failed to read layer range: {}", line));
        let range = range_token.parse().expect(&format!("Failed to parse layer range: {}", range_token));
        Layer::new(depth, range)
    }
}

enum PacketState { Moving(u32), Standing}

struct Firewall
{
    layers: HashMap<u32, Layer>,
    packet_state: PacketState,
}

impl Firewall
{
    fn new() -> Firewall
    {
        Firewall{layers: HashMap::new(), packet_state: PacketState::Standing}
    }

    fn add_layer(self: &mut Self, line: &str)
    {
        let layer = Layer::parse(line);
        self.layers.insert(layer.depth, layer);
    }


    #[cfg(test)]
    fn from_str(lines: &str) -> Firewall
    {
        let mut firewall = Firewall::new();
        for line in lines.split('\n')
        {
            firewall.add_layer(line);
        }
        firewall
    }

    fn from_file(path: &str) -> Firewall
    {
        let mut firewall = Firewall::new();
        let file = File::open(path).expect("Failed to open connections file"); 
        let reader = BufReader::new(file); 
        for line in reader.lines().filter_map(|res| res.ok())
        {
            firewall.add_layer(&line);
        }
        firewall
    }

    fn move_scanner(self: &mut Self)
    {
        for layer in self.layers.values_mut()
        {
            layer.move_scanner();
        }
    }

    fn move_packet(self: &mut Self)
    {
        match self.packet_state
        {
            PacketState::Standing => self.packet_state = PacketState::Moving(0),
            PacketState::Moving(pos) => self.packet_state = PacketState::Moving(pos + 1)
        }
    }

    fn caught(self: &Self) -> Caught
    {
        match self.packet_state
        {
            PacketState::Standing => Caught::No,
            PacketState::Moving(pos) => {
                if let Some(layer) = self.layers.get(&pos)
                {
                    layer.caught()
                }
                else
                {
                    Caught::No
                }
            }
        }
    }

    fn severity(self: &mut Self) -> u32
    {
        let mut severity = 0;
        let max_depth = self.max_depth();

        for _ in 0 .. (max_depth + 1)
        {
            self.move_packet();
            match self.caught()
            {
                Caught::Yes(s) =>{
                    severity += s;
                 },
                Caught::No => {
                }
            }

            self.move_scanner();
        }

        severity
    }

    fn max_depth(self: &Self) -> u32
    {
        self.layers.keys().cloned().max().unwrap()
    }

    fn caught_delay(self: &Self, mut delay: u32) -> bool
    {
        for depth in 0.. (self.max_depth() + 1){
            if let Some(layer) = self.layers.get(&depth) {
                if layer.scanner_pos(delay) == 0 {
                    return true
                }    
            }
            delay += 1;
        }
        false
    }

    fn delay(self: &mut Self) -> u32
    {
        let mut delay = 0;
        while self.caught_delay(delay)
        {
            delay+=1;
        }
        if delay % 1000 == 0
        {
            println!("delay: {}", delay)
        }
        return delay
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_parsing()
    {
        let lines = "0: 3\n1: 2\n4: 4\n6: 4";
        let firewall = Firewall::from_str(lines);
        assert!( match firewall.packet_state {
            PacketState::Standing => true,
            _ => false
        });
        assert_eq!(firewall.layers.get(&0).unwrap().range, 3);
        assert_eq!(firewall.layers.get(&1).unwrap().range, 2);
        assert!(match firewall.layers.get(&2)
                {
                    None => true,
                    _ => false
                });
        assert!(match firewall.layers.get(&3)
                {
                    None => true,
                    _ => false
                });
        assert_eq!(firewall.layers.get(&4).unwrap().range, 4);
        assert!(match firewall.layers.get(&5)
                {
                    None => true,
                    _ => false
                });
        assert_eq!(firewall.layers.get(&6).unwrap().range, 4);
    }

    #[test]
    fn test_severity()
    {
        let lines = "0: 3\n1: 2\n4: 4\n6: 4";
        let mut firewall = Firewall::from_str(lines);
        assert_eq!(firewall.severity(), 24);

        let mut firewall = Firewall::from_str(lines);
        firewall.layers.get_mut(&0).unwrap().scanner_pos = 2;
        firewall.layers.get_mut(&0).unwrap().scanner_dir = Dir::Down;
        firewall.layers.get_mut(&1).unwrap().scanner_pos = 0;
        firewall.layers.get_mut(&1).unwrap().scanner_dir = Dir::Down;
        firewall.layers.get_mut(&4).unwrap().scanner_pos = 2;
        firewall.layers.get_mut(&4).unwrap().scanner_dir = Dir::Up;
        firewall.layers.get_mut(&6).unwrap().scanner_pos = 2;
        firewall.layers.get_mut(&6).unwrap().scanner_dir = Dir::Up;
        assert_eq!(firewall.severity(), 0);
    }

    #[test]
    fn test_scanner_pos()
    {
        let lines = "0: 3\n1: 2\n4: 4\n6: 4";
        let firewall = Firewall::from_str(lines);
        assert_eq!(firewall.layers[&0].scanner_pos(0), 0);
        assert_eq!(firewall.layers[&0].scanner_pos(1), 1);
        assert_eq!(firewall.layers[&0].scanner_pos(2), 2);
        assert_eq!(firewall.layers[&0].scanner_pos(3), 1);
        assert_eq!(firewall.layers[&0].scanner_pos(4), 0);
        assert_eq!(firewall.layers[&0].scanner_pos(5), 1);
        assert_eq!(firewall.layers[&0].scanner_pos(6), 2);
        assert_eq!(firewall.layers[&0].scanner_pos(7), 1);
        assert_eq!(firewall.layers[&0].scanner_pos(8), 0);

        assert_eq!(firewall.layers[&1].scanner_pos(0), 0);
        assert_eq!(firewall.layers[&1].scanner_pos(1), 1);
        assert_eq!(firewall.layers[&1].scanner_pos(2), 0);
        assert_eq!(firewall.layers[&1].scanner_pos(3), 1);
        assert_eq!(firewall.layers[&1].scanner_pos(4), 0);
        assert_eq!(firewall.layers[&1].scanner_pos(5), 1);
        assert_eq!(firewall.layers[&1].scanner_pos(6), 0);
        assert_eq!(firewall.layers[&1].scanner_pos(7), 1);
        assert_eq!(firewall.layers[&1].scanner_pos(8), 0);

        assert_eq!(firewall.layers[&6].scanner_pos(0), 0);
        assert_eq!(firewall.layers[&6].scanner_pos(1), 1);
        assert_eq!(firewall.layers[&6].scanner_pos(2), 2);
        assert_eq!(firewall.layers[&6].scanner_pos(3), 3);
        assert_eq!(firewall.layers[&6].scanner_pos(4), 2);
        assert_eq!(firewall.layers[&6].scanner_pos(5), 1);
        assert_eq!(firewall.layers[&6].scanner_pos(6), 0);
        assert_eq!(firewall.layers[&6].scanner_pos(7), 1);
        assert_eq!(firewall.layers[&6].scanner_pos(8), 2);
        assert_eq!(firewall.layers[&6].scanner_pos(9), 3);
        assert_eq!(firewall.layers[&6].scanner_pos(10), 2);
        assert_eq!(firewall.layers[&6].scanner_pos(11), 1);
        assert_eq!(firewall.layers[&6].scanner_pos(12), 0);

        assert_eq!(firewall.layers[&6].scanner_pos(288), 0);
    }

    #[test]
    fn caught_delay()
    {
        let lines = "0: 3\n1: 2\n4: 4\n6: 4";
        let firewall = Firewall::from_str(lines);
        assert_eq!(firewall.caught_delay(0), true);
        assert_eq!(firewall.caught_delay(1), true);
        assert_eq!(firewall.caught_delay(2), true);
        assert_eq!(firewall.caught_delay(3), true);
        assert_eq!(firewall.caught_delay(4), true);
        assert_eq!(firewall.caught_delay(5), true);
        assert_eq!(firewall.caught_delay(6), true);
        assert_eq!(firewall.caught_delay(7), true);
        assert_eq!(firewall.caught_delay(8), true);
        assert_eq!(firewall.caught_delay(9), true);
        assert_eq!(firewall.caught_delay(10), false);
    }
}
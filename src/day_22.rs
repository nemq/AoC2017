use std::collections::HashMap;
use std::fmt;

pub fn first_puzzle() -> String {
    let grid = Grid::initialize(
".########.....#...##.####
....#..#.#.##.###..#.##..
##.#.#..#.###.####.##.#..
####...#...####...#.##.##
..#...###.#####.....##.##
..#.##.######.#...###...#
.#....###..##....##...##.
##.##..####.#.######...##
#...#..##.....#..#...#..#
........#.##..###.#.....#
#.#..######.#.###..#...#.
.#.##.##..##.####.....##.
.....##..#....#####.#.#..
...#.#.#..####.#..###..#.
##.#..##..##....#####.#..
.#.#..##...#.#####....##.
.####.#.###.####...#####.
...#...######..#.##...#.#
#..######...#.####.#..#.#
...##..##.#.##.#.#.#....#
###..###.#..#.....#.##.##
..#....##...#..#..##..#..
.#.###.##.....#.###.#.###
####.##...#.#....#..##...
#.....#.#..#.##.#..###..#");

    let mut carrier = VirusCarier::new(grid);
    for _ in 0..10000 {
        carrier.burst();
    }
    format!("{}", carrier.infections)
}

pub fn second_puzzle() -> String {
   let grid = Grid::initialize(
".########.....#...##.####
....#..#.#.##.###..#.##..
##.#.#..#.###.####.##.#..
####...#...####...#.##.##
..#...###.#####.....##.##
..#.##.######.#...###...#
.#....###..##....##...##.
##.##..####.#.######...##
#...#..##.....#..#...#..#
........#.##..###.#.....#
#.#..######.#.###..#...#.
.#.##.##..##.####.....##.
.....##..#....#####.#.#..
...#.#.#..####.#..###..#.
##.#..##..##....#####.#..
.#.#..##...#.#####....##.
.####.#.###.####...#####.
...#...######..#.##...#.#
#..######...#.####.#..#.#
...##..##.#.##.#.#.#....#
###..###.#..#.....#.##.##
..#....##...#..#..##..#..
.#.###.##.....#.###.#.###
####.##...#.#....#..##...
#.....#.#..#.##.#..###..#");

    let mut carrier = VirusCarier::new(grid);
    for _ in 0..10000000 {
        carrier.burst_evolved();
    }
    format!("{}", carrier.infections)
}

enum Direction {Top, Bottom, Left, Right}
#[derive(Clone)]
enum Node {Clean, Weakened, Infected, Flagged}

struct Grid {
    infected: HashMap<(i64, i64), Node>,
}



impl Grid {
    fn new() -> Grid {
        Grid {infected: HashMap::new()}
    }

    fn initialize(map: &str) -> Grid {
        let mut grid = Grid::new();
        let size = map.split('\n').count();
        let offset  = ((size - 1) / 2) as i64;
        for (row, line) in map.split('\n').map(|l| l.trim()).enumerate() {
           for (col, ch) in line.chars().enumerate() {
               let pos = (col as i64 - offset, offset - row as i64 );
               match ch {
                   '#' => {grid.set(&pos, Node::Infected);},
                   _ => {}
               };
           } 
        }
        grid
    }

    fn get(&self, pos: &(i64, i64)) -> Node {
        if let Some(node) = self.infected.get(pos) {
            node.clone()
        }
        else {
            Node::Clean
        }
    }

    fn set(&mut self, pos: &(i64, i64), node: Node) {
        match node {
            Node::Clean => {self.infected.remove(pos);}
            _ => {self.infected.insert(pos.clone(), node);}
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut min_x = i64::max_value();
        let mut max_x = i64::min_value();
        let mut min_y = i64::max_value();
        let mut max_y = i64::min_value();

        for (&(x,y), _) in self.infected.iter() {
            min_x = i64::min(min_x, x);
            max_x = i64::max(max_x, x);
            min_y = i64::min(min_y, y);
            max_y = i64::max(max_y, y);
        }

        let offset_x = i64::max(max_x.abs(), min_x.abs());
        let offset_y = i64::max(max_y.abs(), min_y.abs());

        let width = 2 * offset_x + 1;
        let height = 2 * offset_y + 1;
        
        for y in 0.. height {
            for x in 0.. width {
                let pos = (x - offset_x, offset_y - y);
                match self.get(&pos) {
                    Node::Clean => write!(f, ".")?,
                    Node::Flagged => write!(f, "F")?,
                    Node::Weakened => write!(f, "W")?,
                    Node::Infected => write!(f, "#")?
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "")
    }
}


struct VirusCarier {
    grid: Grid,
    pos: (i64, i64),
    dir: Direction,
    infections: u64,
}

impl VirusCarier {
    fn new(grid: Grid) -> VirusCarier {
        VirusCarier {grid: grid, pos: (0,0), dir: Direction::Top, infections: 0}
    }

    fn burst(&mut self) {

        match self.grid.get(&self.pos) {
            Node::Infected => {
                self.turn_right();
                self.grid.set(&self.pos, Node::Clean);
                self.move_forward();
            }, 
            Node::Clean => {
                self.turn_left();
                self.grid.set(&self.pos, Node::Infected);
                self.infections += 1;
                self.move_forward();
            }
            _ => {}
        }
    }

    fn burst_evolved(&mut self) {
        match self.grid.get(&self.pos) {
            Node::Clean => {
                self.grid.set(&self.pos, Node::Weakened);
                self.turn_left();
            },
            Node::Weakened => {
                self.grid.set(&self.pos, Node::Infected);
                self.infections += 1;
            },
            Node::Infected => {
                self.grid.set(&self.pos, Node::Flagged);
                self.turn_right();
            },
            Node::Flagged => {
                self.grid.set(&self.pos, Node::Clean);
                self.turn_back();
            }
        }
        self.move_forward();
    }

    fn turn_left(&mut self) {
        match self.dir {
            Direction::Top => self.dir = Direction::Left,
            Direction::Right => self.dir = Direction::Top,
            Direction::Bottom => self.dir = Direction::Right,
            Direction::Left => self.dir = Direction::Bottom,
        }
    }

    fn turn_right(&mut self) {
        self.turn_left();
        self.turn_left();
        self.turn_left();
    }

    fn turn_back(&mut self) {
        self.turn_left();
        self.turn_left();
    }

    fn move_forward(&mut self) {
        let (x, y) = self.pos;
        match self.dir {
            Direction::Top => self.pos = (x, y + 1),
            Direction::Right => self.pos = (x + 1, y),
            Direction::Bottom => self.pos = (x, y -1),
            Direction::Left => self.pos = (x - 1, y),
        }
    }

}



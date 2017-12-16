use std::i32;
use std::io::prelude::*; 
use std::fs::File; 

pub fn first_puzzle() -> String
{
    let coord = read_path("path.txt");
    format!("{}", coord.distance_from_origin())
}

pub fn second_puzzle() -> String
{
    let mut file = File::open("path.txt").expect("Failed to open path file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read path file");
    let mut coord = HexagonCoord::origin();
    let mut max_dist = 0i32;
    for dir in content.split_terminator(',')
    {
        coord.offset(dir.trim());
        max_dist = i32::max(coord.distance_from_origin(), max_dist);
    }

    format!("{}", max_dist)
}

fn read_path(path: &str) -> HexagonCoord
{
    let mut file = File::open(path).expect("Failed to open path file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read path file");
    let mut coord = HexagonCoord::origin();
    for dir in content.split_terminator(',')
    {
        coord.offset(dir.trim());
    }

    coord
}

struct HexagonCoord
{
    x: i32,
    y: i32,
    z: i32
}

impl HexagonCoord
{
    fn new(x: i32, y: i32, z: i32) -> HexagonCoord
    {
        HexagonCoord {x, y, z}
    }

    fn origin() -> HexagonCoord
    {
        HexagonCoord::new(0, 0, 0)
    }

    fn offset(self: &mut Self, direction: &str)
    {
        match direction
        {
            "n" => {
                self.y += 1;
                self.z -= 1;
            },
            "ne" => {
                self.x += 1;
                self.z -= 1;
            },
            "nw" => {
                self.x -= 1;
                self.y += 1;
            },
            "s" => {
                self.y -= 1;
                self.z += 1;
            },
            "se" => {
                self.x += 1;
                self.y -= 1;
            },
            "sw" => {
                self.x -= 1;
                self.z += 1;
            },
            _ => {

                panic!("Invalid direciton: {}", direction);
            }
        }
    }

    fn distance(self: &Self, other: &HexagonCoord) -> i32
    {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z + other.z).abs()) / 2
    }

    fn distance_from_origin(self: &Self) -> i32
    {
        self.distance(&HexagonCoord::origin())
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_distance_from_origin()
    {
        let mut coord = HexagonCoord::origin();
        for dir in "ne,ne,ne".split_terminator(',')
        {
            coord.offset(dir);
        }
        assert_eq!(coord.distance_from_origin(), 3);

        let mut coord = HexagonCoord::origin();
        for dir in "ne,ne,sw,sw".split_terminator(',')
        {
            coord.offset(dir);
        }
        assert_eq!(coord.distance_from_origin(), 0);

        let mut coord = HexagonCoord::origin();
        for dir in "ne,ne,s,s".split_terminator(',')
        {
            coord.offset(dir);
        }
        assert_eq!(coord.distance_from_origin(), 2);

        let mut coord = HexagonCoord::origin();
        for dir in "se,sw,se,sw,sw".split_terminator(',')
        {
            coord.offset(dir);
        }
        assert_eq!(coord.distance_from_origin(), 3);
    }
}
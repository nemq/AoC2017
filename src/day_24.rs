use std::fs::File;
use std::io::prelude::*;
use simple_error::SimpleError;
use std::error::Error;
use std::collections::HashSet;

pub fn first_puzzle() -> String {
    let graph = Graph::from_file("bridge.txt").unwrap();
    format!("{}", graph.find_max_strength(0))
}

pub fn second_puzzle() -> String {
    let graph = Graph::from_file("bridge.txt").unwrap();
    let max_length = graph.find_max_length(0);
    format!("{}", graph.find_max_strength_with_length(0, max_length))
}

struct Graph {
    edges: Vec<(u64, u64)>
}

impl Graph {
    fn new() -> Graph {
        Graph {
            edges: Vec::new()
        }
    }

    fn from_str(lines: &str) -> Result<Graph, Box<Error>> {
        let mut graph = Graph::new();
        for line in lines.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()) {
            graph.parse(line)?;
        }
        Ok(graph)
    }

    fn from_file(path: &str) -> Result<Graph, Box<Error>> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Graph::from_str(&buffer)
    }

    fn parse(&mut self, line: &str) -> Result<(), Box<Error>> {
        let mut iter = line.split('/');
        let first_token = iter.next()
                              .ok_or(SimpleError::new(&format!("missing first port: {}", line)))?;
        let first_node = first_token.parse::<u64>()?;

        let second_token = iter.next()
                               .ok_or(SimpleError::new(&format!("missing second port: {}", line)))?;
        let second_node = second_token.parse::<u64>()?;
        self.edges.push((first_node, second_node));
        Ok(())
    }

    fn find_max_strength(&self, node: u64) -> u64 {
        self.max_strength(node, HashSet::new())
    }

    fn find_max_length(&self, node: u64) -> u64 {
        self.max_length(node, HashSet::new())
    }

    fn find_max_strength_with_length(&self, node: u64, length: u64) -> u64 {
        self.max_strength_with_length(node, length, HashSet::new())
    }

    fn max_strength(&self, node: u64, used: HashSet<usize>) -> u64 {
        let compatible = self.compatible(node, &used);
        compatible.into_iter().map(|((x, y), u)| x + y + self.max_strength(y, u)).max().unwrap_or(0)
    }

    fn max_length(&self, node: u64, used: HashSet<usize>) -> u64 {
        let compatible = self.compatible(node, &used);
        compatible.into_iter().map(|((_, y), u)| 1 + self.max_length(y, u)).max().unwrap_or(0)
    }

    fn max_strength_with_length(&self, node: u64, length: u64, used: HashSet<usize>) -> u64 {
        let compatible_max_length = 
        self.compatible(node, &used)
        .into_iter()
        .filter(|&((_, y), ref u)| self.max_length(y, u.clone()) == length -1)
        .collect::<Vec<_>>();

        compatible_max_length
        .iter()
        .map(|&((x, y), ref u)| x + y + self.max_strength_with_length(y, length -1, u.clone()))
        .max()
        .unwrap_or(0)
    }

    fn compatible(&self, node: u64, used: &HashSet<usize>) -> Vec<((u64, u64), HashSet<usize>)> {
        self.edges
            .iter()
            .enumerate()
            .filter_map(|(idx, &(x,y)) | 
                if used.contains(&idx) {
                    None
                }
                else if x == node {
                    let mut u = used.clone();
                    u.insert(idx);
                    Some(((x,y), u))
                }
                else if y == node {
                    let mut u = used.clone();
                    u.insert(idx);
                    Some(((y, x), u))
                }
                else {
                    None
                })
            .collect::<Vec<_>>()
    }
}


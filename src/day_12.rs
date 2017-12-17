use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*; 
use std::io::BufReader; 
use std::fs::File; 

pub fn first_puzzle() -> String
{
    let graph = Graph::from_file("connections.txt");
    format!("{}", graph.connections(0).len())
}

pub fn second_puzzle() -> String
{
    let graph = Graph::from_file("connections.txt");
    format!("{}", graph.groups().len())
}

struct Node
{
    id: u32,
    connections: Vec<u32>
}

impl Node
{
    fn new(id: u32) -> Node
    {
        Node {id, connections: Vec::new()}
    }

    fn parse(record: &str) -> Node
    {
        let mut iter = record.split("<->").map(|token| token.trim());
        let token = iter.next().expect(&format!("Error reading id: {}", record));
        let id = token.parse().expect(&format!("Error parsing id: {}", token));
        let mut node = Node::new(id);
        let token = iter.next().expect(&format!("Error reading connections: {}", record));
        for conn in token.split(',').map(|token| token.trim())
        {
           node.connections.push(conn.parse().expect(&format!("Error parsing connection id: {}", conn))); 
        }
        node
    }
}

struct Graph
{
    nodes: HashMap<u32, Node>
}

impl Graph
{

    fn new() ->Graph
    {
        Graph{nodes: HashMap::new()}
    }

    fn add(self: &mut Self, node: Node)
    {
        self.nodes.insert(node.id, node);
    }

    fn connections(self: &Self, id: u32) -> HashSet<u32>
    {
        let mut connected = HashSet::new();
        connected.insert(id);
        let mut visited = HashSet::new();
        self.find_connections(id, &mut visited, &mut connected);
        connected
    }

    fn find_connections(self: &Self, id: u32, visited: &mut HashSet<u32>, connected: &mut HashSet<u32>)
    {
        visited.insert(id);
        for conn_id in self.nodes.get(&id).unwrap().connections.iter()
        {
            if visited.contains(&conn_id)
            {
                continue;
            }
            connected.insert(*conn_id);
            self.find_connections(*conn_id, visited, connected);
        }
    }

    fn groups(self: &Self) -> Vec<HashSet<u32>>
    {
        let mut groups = Vec::new();
        let mut available = self.nodes.keys().cloned().collect::<HashSet<u32>>();
        while !available.is_empty()
        {
            let node = *available.iter().next().unwrap();
            let group = self.connections(node);
            available = available.difference(&group).cloned().collect();
            groups.push(group);
        }
        groups
    }

    fn from_file(path: &str) -> Graph
    {
        let mut graph = Graph::new();
        let file = File::open(path).expect("Failed to open connections file"); 
        let reader = BufReader::new(file); 
        for line in reader.lines().filter_map(|res| res.ok())
        {
            graph.add(Node::parse(&line));
        }
        graph
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_graph()
    {
        let mut graph = Graph::new();
        graph.add(Node::parse("0 <-> 2"));
        graph.add(Node::parse("1 <-> 1"));
        graph.add(Node::parse("2 <-> 0, 3, 4"));
        graph.add(Node::parse("3 <-> 2, 4"));
        graph.add(Node::parse("4 <-> 2, 3, 6"));
        graph.add(Node::parse("5 <-> 6"));
        graph.add(Node::parse("6 <-> 4,5"));
        let cons = graph.connections(0);
        assert_eq!(cons.len(), 6);
        let groups = graph.groups();
        assert_eq!(groups.len(), 2);
    }
}
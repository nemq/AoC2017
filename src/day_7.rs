use std::collections::HashSet;
use std::collections::HashMap;
use std::io::prelude::*; 
use std::io::BufReader; 
use std::fs::File; 


pub fn first_puzzle() -> String
{
    let root = read_tree("tower.txt").unwrap();
    format!("{}", root.name)
}

pub fn second_puzzle() -> String
{
    let root = read_tree("tower.txt").unwrap();
    format!("{}", root.balance())
}


fn read_tree(path: &str) -> Result<Node, &'static str>
{
    let mut parser = TreeParser::new();
    let file = File::open(path).expect("Failed to open tower file"); 
    let reader = BufReader::new(file); 
    for line in reader.lines().filter_map(|res| res.ok())
    {
        parser.parse(&line);
    }

    parser.build_tree()
}

struct Node
{
    name: String,
    weight: u32,
    children: Vec<Node>
}

impl Node
{
    pub fn new(name: &str, weight: u32) -> Node
    {
        Node{
            name: String::from(name),
            weight: weight,
            children: Vec::new()
        }
    }

    pub fn add_child(self: &mut Self, child: Node)
    {
        self.children.push(child);
    }

    pub fn branch_weight(self: &Self) -> u32
    {
        let mut weight = 0;
        self.count_branch_weight(&mut weight);
        weight
    }

    fn count_branch_weight(self: &Self, sum: &mut u32)
    {
        *sum += self.weight;
        for child in self.children.iter()
        {
            child.count_branch_weight(sum);
        }
    }

    fn is_balanced(self: &Self) -> bool
    {
        let different_weights = self.children.iter()
                                             .map(|child| child.branch_weight())
                                             .collect::<HashSet<_>>();
        different_weights.len() == 1 || different_weights.len() == 0
    }

    fn find_unbalanced(self: &Self) -> &Node
    {
        for child in self.children.iter().filter(|ch| !ch.is_balanced())
        {
            return child.find_unbalanced();
        }

        self
    }

    fn balance(self: &Self) -> i32
    {
        let node = self.find_unbalanced();
        let balanced_subbranch_weight = node.balanced_subbranch_weight();
        for child in node.children.iter()
        {
            let subbranch_weight = child.branch_weight();
            if subbranch_weight != balanced_subbranch_weight
            {
                let diff = balanced_subbranch_weight as i32 - subbranch_weight as i32;
                return child.weight as i32 + diff 
            }            
        }

        0
    }

    fn balanced_subbranch_weight(self: &Self) -> u32
    {
        let mut weight_count = HashMap::new();
        for weight in self.children.iter().map(|ch| ch.branch_weight())
        {
            let count = weight_count.entry(weight).or_insert(0);
            *count += 1;
        }

        match weight_count.len()
        {
            0 => 0,
            1 => *weight_count.keys().nth(0).unwrap(),
            2 => 
            {
                let (&first_weight, &first_count) = weight_count.iter().nth(0).unwrap();
                let (&second_weight, &second_count) = weight_count.iter().nth(1).unwrap();
                match (first_count, second_count)
                {
                    (1, 1) => panic!("Ambigious which weight should be choosed (should be handled)"),
                    (1, _) => second_weight,
                    (_, 1) => first_weight,
                    (_, _) => panic!("Assumed single node with wrong weight")
                }
            }
            _ => panic!("Assumed single node with wrong weight")
        }
    }
}

struct TreeParser
{
    nodes: HashMap<String, u32>,
    edges: HashMap<String, Vec<String>>,
}

impl TreeParser
{
    pub fn new() -> TreeParser
    {
        TreeParser
        {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn build_tree(self: Self ) -> Result<Node, &'static str>
    {
        let mut root = self.find_root()?;
        self.add_children(&mut root);

        Ok(root)
    }

    fn add_children(self: &Self, node: &mut Node)
    {
        if let Some(children) = self.edges.get(&node.name) {
            for name in children.iter()
            {
                let mut child = Node::new(name, *self.nodes.get(name).unwrap());
                self.add_children(&mut child);
                node.add_child(child);
            }
        }
    }

    fn find_root(self: &Self) -> Result<Node, &'static str>
    {
        let parents = self.edges.keys().collect::<HashSet<_>>();
        let children = self.edges.values()
                                 .flat_map(|v| v.iter())
                                 .collect::<HashSet<_>>();

        let roots = parents.difference(&children).cloned().collect::<Vec<_>>();
        if roots.len() == 0
        {
            Err("No root found.")
        }
        else if roots.len() > 1
        {
            Err("More than one root found.")
        }
        else
        {
            let name = roots.get(0).unwrap();
            let weight = self.nodes.get(*name).unwrap();
            Ok(Node::new(name, *weight))
        }
    }

    pub fn parse(self: &mut Self, line: &str) -> Result<(), &str>
    {
        let mut name = String::new();
        let mut weight = 0;
        for (idx, token) in line.split_whitespace().enumerate() 
        {
            match idx {
                0 => 
                { 
                    name = String::from(token) 
                },
                1 => 
                { 
                    if let Ok(w) = token.trim_matches(|ch| ch == '(' || ch == ')')
                                        .parse::<u32>() 
                    {
                        weight = w;     
                    }
                    else
                    {
                        return Err("Failed to read height")
                    }
                },
                2 if token == "->" => {},
                2 if token != "->" => 
                { 
                    return Err("Third token is not equal to '->'")
                },
                _ => 
                { 
                   let child_name = String::from(token.trim_right_matches(','));
                   let entry = self.edges.entry(name.clone()).or_insert(Vec::new());
                   entry.push(child_name.clone());
                }
            }
        }
        self.nodes.insert(name, weight);
        Ok(())
    }
}

#[cfg(test)] 
mod tests  
{ 
    use super::*; 

    static sample: &str = 
"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn first_puzzle() 
    {
       let mut parser = TreeParser::new();
       for line in sample.split_terminator('\n')
       {
           parser.parse(line);
       }

       let root = parser.build_tree().unwrap();
       assert_eq!("tknk", root.name);
       assert_eq!(41, root.weight);
       assert_eq!(3, root.children.len());
       assert_eq!(778, root.branch_weight());
       assert_eq!(false, root.is_balanced());

    }

    #[test]
    fn second_puzzle() 
    {
       let mut parser = TreeParser::new();
       for line in sample.split_terminator('\n')
       {
           parser.parse(line);
       }

       let root = parser.build_tree().unwrap();
       let unbalanced = root.find_unbalanced();
       assert_eq!("tknk", unbalanced.name);
       assert_eq!(41, unbalanced.weight);
       assert_eq!(3, unbalanced.children.len());
       assert_eq!(778, unbalanced.branch_weight());
       assert_eq!(false, unbalanced.is_balanced());
       assert_eq!(unbalanced.balanced_subbranch_weight(), 243);
       assert_eq!(root.balance(), 60);
    }
}


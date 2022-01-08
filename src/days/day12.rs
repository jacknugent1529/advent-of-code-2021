use std::fs;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let graph: Graph = file_str.parse().unwrap();
    
    // println!("{:?}", graph);
    let res = get_valid_paths_a(&graph, "start".to_string(), "end".to_string());
    Ok(res)
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let graph: Graph = file_str.parse().unwrap();
    
    // println!("{:?}", graph);
    let res = get_valid_paths_b(&graph, "start".to_string(), "end".to_string());
    Ok(res)
}

fn get_valid_paths_a(graph: &Graph, start: String, end: String) -> i32 {
    let mut start_path = Path::new();
    start_path.add_node(start.clone(), false);
    return get_valid_paths_a_r(graph, &mut start_path, start, end);
}
fn get_valid_paths_a_r(graph: &Graph, path: &mut Path, start: String, end: String) -> i32 {
    // recursive is easier here
    // println!("path at start: {:?}", path);
    // println!("last: {:?}", path.last());
    if path.last().clone() == end {
        // println!("valid path: {:?}", path);
        return 1;
    }
    let mut total = 0;
    match graph.get_adjacent(path.last()) {
        Some(adj_nodes) => {
            for node in adj_nodes {
                if path.add_node(node.clone(), false) {
                    // println!("path after add: {:?}", path);
                    total += get_valid_paths_a_r(graph, path, start.clone(), end.clone());
                    path.pop();
                }
            }    
        },
        None => { return 0 }
    }
    // println!("path at end: {:?}", path);
    return total;
}

fn get_valid_paths_b(graph: &Graph, start: String, end: String) -> i32 {
    let mut start_path = Path::new();
    start_path.add_node(start.clone(), false);
    return get_valid_paths_b_r(graph, &mut start_path, start, end);
}
fn get_valid_paths_b_r(graph: &Graph, path: &mut Path, start: String, end: String) -> i32 {
    // recursive is easier here
    // println!("path at start: {:?}", path);
    // println!("last: {:?}", path.last());
    if path.last().clone() == end {
        // println!("valid path: {:?}", path);
        return 1;
    }
    let mut total = 0;
    match graph.get_adjacent(path.last()) {
        Some(adj_nodes) => {
            for node in adj_nodes {
                if path.add_node(node.clone(), true) {
                    // println!("path after add: {:?}", path);
                    total += get_valid_paths_b_r(graph, path, start.clone(), end.clone());
                    path.pop();
                }
            }    
        },
        None => { return 0 }
    }
    // println!("path at end: {:?}", path);
    return total;
}

#[derive(Debug)]
struct Path {
    path: Vec<String>,
    small_caves: HashSet<String>,
    visited_twice: Option<String>
}

impl Path {
    fn new() -> Self {
        return Path {
            path: vec![],
            small_caves: HashSet::new(),
            visited_twice: None
        };
    }
    fn add_node(&mut self, node: String, visit_twice: bool) -> bool {
        let is_univisited_small_cave = !self.small_caves.contains(&node);
        if  is_univisited_small_cave || (&node != "start" && visit_twice && self.visited_twice.is_none()) {
            let lower_case: Vec<char> = ('a'..='z').collect();
            let is_lower_case = |s: &String| s.chars().all(|c| lower_case.contains(&c));
            if !is_univisited_small_cave && is_lower_case(&node) {
                // println!("visited twice: {}", node);
                self.visited_twice = Some(node.clone());
            }
            if is_lower_case(&node) {
                self.small_caves.insert(node.clone());
            }
            self.path.push(node);
            return true;
        }
        return false;
    }
    
    fn is_valid(&self) -> bool {
        return self.path[0] == "start" && self.path[self.path.len()-1] == "end";
    }
    
    fn last(&self) -> &String {
        return &self.path[self.path.len() - 1];
    }
    
    fn pop(&mut self) -> bool {
        let maybe_removed = self.path.pop();
        if maybe_removed == self.visited_twice {
            self.visited_twice = None;
            return maybe_removed.is_some();
        }
        if let Some(removed) = maybe_removed {
            self.small_caves.remove(&removed);
            return true;
        }
        return false
    }
}

#[derive(Debug)]
struct Graph {
    // maps node to adjacency list
    connections: HashMap<String, Vec<String>>,
}

impl Graph {
    fn get_adjacent(&self, node: &String) -> Option<&Vec<String>> {
        return self.connections.get(node);
    }
}

impl FromStr for Graph {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        for l in s.lines() {
            let ab: Vec<&str> = l.split("-").take(2).collect();
            let a = ab[0].to_string();
            let b = ab[1].to_string();

            let adj_to_a = connections.entry(a.clone()).or_insert(vec![]);
            adj_to_a.push(b.clone());
            let adj_to_b = connections.entry(b).or_insert(vec![]);
            adj_to_b.push(a);

        }
        Ok(Graph{
            connections
        })
    }
}
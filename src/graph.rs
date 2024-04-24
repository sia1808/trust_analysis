use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;
#[derive(Debug)]


pub struct Graph {
    pub connections: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn num_nodes(&self) -> usize {
        self.connections.len()
    }
    pub fn shortest_paths_from(&self, source:usize) -> HashMap<usize,usize> {
        let mut order = VecDeque::new();
        let mut distance = HashMap::new();
        let mut already_visited = HashMap::new();

        order.push_back(source);
        distance.insert(source,0);
        already_visited.insert(source,true);

        while let Some(node) = order.pop_front() {
            let current_dist = *distance.get(&node).unwrap_or(&0);

            for &connection in self.connections.get(&node).unwrap_or(&vec![]) {
                if !already_visited.contains_key(&connection)  {
                    already_visited.insert(connection, true);
                    distance.insert(connection, current_dist + 1);
                    order.push_back(connection);
                }
            }
        }

        return distance
    }
}



pub fn read_graph(filename: &str) -> Option<Graph> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let num_nodes = lines.next()?.unwrap().parse::<usize>().ok()?;
    let mut connections = HashMap::new();

    for line in lines.filter_map(|line| line.ok()) {
        let mut parts = line.split_whitespace();
        let one = parts.next()?.parse::<usize>().ok()?;
        let two = parts.next()?.parse::<usize>().ok()?;

        connections.entry(one).or_insert(Vec::new()).push(two);
    }

    return Some(Graph {connections})
}

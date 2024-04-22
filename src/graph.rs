use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
#[derive(Debug)]


pub struct Graph {
    pub connections: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn num_nodes(&self) -> usize {
        self.connections.len()
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

    Some(Graph {connections})
}

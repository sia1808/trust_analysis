use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;
#[derive(Debug)]


pub struct Graph { //making it public
    pub connections: HashMap<usize, Vec<usize>>, // HashMap to store connections between nodes
}

impl Graph {
    // Function to get the number of nodes in the graph
    pub fn num_nodes(&self) -> usize {
        self.connections.len()
    }
    //finding shortest paths from a given source node
    pub fn shortest_paths_from(&self, source:usize) -> HashMap<usize,usize> {
        let mut order = VecDeque::new();
        let mut distance = HashMap::new(); // HashMap to store distances from source
        let mut already_visited = HashMap::new(); // HashMap to track visited nodes

        order.push_back(source);
        distance.insert(source,0); //setting distance to 0
        already_visited.insert(source,true); //marking visited nodes to be true

        // Now doing BFS algorithm to find the shortest path 
        while let Some(node) = order.pop_front() {
            let current_dist = *distance.get(&node).unwrap_or(&0); // Current distance from source

            // Iterating over the connections of the current node
            for &connection in self.connections.get(&node).unwrap_or(&vec![]) {
                if !already_visited.contains_key(&connection)  {
                    already_visited.insert(connection, true);
                    distance.insert(connection, current_dist + 1); // Updating distance from source
                    order.push_back(connection); // adding the connection to the order
                }
            }
        }

        return distance
    }
}

// Function to read graph from a file
pub fn read_graph(filename: &str) -> Option<Graph> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines(); //iterator over the lines

    // Parsing number of nodes from first line
    let num_nodes = lines.next()?.unwrap().parse::<usize>().ok()?;
    let mut connections = HashMap::new(); // HashMap to store connections

    // Doing the rest of the lines
    for line in lines.filter_map(|line| line.ok()) {
        let mut parts = line.split_whitespace();
        let one = parts.next()?.parse::<usize>().ok()?; // first node
        let two = parts.next()?.parse::<usize>().ok()?; // second node

        connections.entry(one).or_insert(Vec::new()).push(two); // adding the connection to hashmap
    }

    return Some(Graph {connections})
}

use crate::graph::Graph;
use std::collections::HashMap;

pub fn calculate_paths(graph: &Graph) -> HashMap <usize, HashMap<usize,usize>> {
    let mut all_shortest_paths = HashMap::new();
    let unique_keys: Vec<usize> = graph.connections.keys().cloned().collect();

    for key in &unique_keys {
        let mut paths = HashMap::new();

        let distance = graph.shortest_paths_from(*key);

        for node in graph.connections.keys() {
            if let Some(dist) = distance.get(node) {
                paths.insert(*node, *dist);
            }
        }
        all_shortest_paths.insert(*key, paths);
    }

    return all_shortest_paths
} 

use crate::graph::Graph;
use std::collections::HashMap;

// Function to calculate shortest paths from all nodes in the graph
pub fn calculate_paths(graph: &Graph) -> HashMap <usize, HashMap<usize,usize>> {
    let mut all_shortest_paths = HashMap::new();
    // Collect unique keys (node IDs) from the graph's connections
    let unique_keys: Vec<usize> = graph.connections.keys().cloned().collect();

    // iterating over each node
    for key in &unique_keys {
        let mut paths = HashMap::new(); // hashmap to store the shortest path from current node
        // Computing shortest paths from the current node to all other nodes
        let distance = graph.shortest_paths_from(*key);

        // iterating through each node to check if a shortest path exists from current node to target node
        for node in graph.connections.keys() {
            if let Some(dist) = distance.get(node) {
                paths.insert(*node, *dist);
            }
        }
        // Inserting the hashmap of shortest paths for the current node into the overall hashmap
        all_shortest_paths.insert(*key, paths);
    }

    return all_shortest_paths
} 

use crate::graph::{Graph, read_graph};
use std::collections::{HashMap, HashSet};

pub fn compute_jaccard_similarity(graph_file: &str, bfs_distance: usize) -> Option<HashMap<(usize, usize), f64>> {
    // Read the graph from the file
    let graph = read_graph(graph_file)?;

    // Compute shortest paths for all nodes in the graph
    let mut jaccard_similarities = HashMap::new();
    for &node1 in graph.connections.keys() {
        let shortest_paths = graph.shortest_paths_from(node1);
        
        // Iterate over all nodes again to find pairs of nodes with specified BFS distance
        for &node2 in graph.connections.keys() {
            if let Some(d) = shortest_paths.get(&node2) {
                if *d == bfs_distance {
                    // Calculate Jaccard similarity between neighborhoods of source and target nodes
                    let jaccard_similarity = calculate_jaccard_similarity(&graph, node1, node2);
                    jaccard_similarities.insert((node1, node2), jaccard_similarity);
                }
            }
        }
    }

    Some(jaccard_similarities)
}

fn calculate_jaccard_similarity(graph: &Graph, node1: usize, node2: usize) -> f64 {
    let neighbors1: HashSet<_> = graph.connections[&node1].iter().cloned().collect();
    let neighbors2: HashSet<_> = graph.connections[&node2].iter().cloned().collect();

    let intersection = neighbors1.intersection(&neighbors2).count() as f64;
    let unions = neighbors1.union(&neighbors2).count() as f64;
    
    if unions == 0.0 {
        0.0 // To handle the case when both neighborhoods are empty
    } else {
        intersection / unions
    }
}
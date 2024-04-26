use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

pub fn calculate_similarity(graph: &Graph) -> HashMap<(usize, usize), f64> {
    let mut similarity_scores = HashMap::new();

    for (node_a, connections_a) in &graph.connections {
        for (node_b, connections_b) in &graph.connections {
            if node_a != node_b {
                let intersections = connections_a
                    .iter()
                    .filter(|&node| connections_b.contains(node))
                    .count();
                let combos = connections_a.len() + connections_b.len() - intersections;
                let similarity = intersections as f64 / combos as f64;
                similarity_scores.insert((*node_a, *node_b), similarity);
            }
        }
    }

    similarity_scores
}
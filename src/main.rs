mod graph;
mod shortestpath; // Import the shortestpath module

use graph::Graph;
use shortestpath::calculate_paths;
use std::collections::HashMap;

fn main() {
    let filename = "test_data.txt";
    if let Some(graph) = graph::read_graph(filename) {
        let all_shortest_paths = calculate_paths(&graph);

        for (key, paths) in &all_shortest_paths {
            println!("Shortest path from node {}", key);
            for (node, dist) in paths {
                println!(" -- Shortest distance to node {} is {}", node, dist);

            }
            println!();
            println!();
        }
    } else {
        println!("Failed to read graph from file error.")
    }
}
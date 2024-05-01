mod graph;
mod shortestpath; 
mod similarityscores;
mod statisticalanalysis;

use graph::Graph;
use similarityscores::compute_jaccard_similarity;
use statisticalanalysis::{find_mean_similarity, find_max_similarity, find_percentage_above_threshold};
use std::collections::HashMap;

use crate::graph::read_graph;
use crate::shortestpath::calculate_paths;

fn main() {
    let filename = "fb_data.txt";
    if let Some(graph) = graph::read_graph(filename) {
        let all_shortest_paths = calculate_paths(&graph);
        println!("Shortest Path Analysis:");
        println!();
        for (key, paths) in &all_shortest_paths {
            println!("Shortest path from node {}", key);
            for (node, dist) in paths {
                println!(" -- Shortest distance to node {} is {}", node, dist);
            }
            println!();
        }
        
        let bfs_distance = 1; // Set the BFS distance you want to compute Jaccard similarity for
        
        match compute_jaccard_similarity(filename, bfs_distance) {
            Some(jaccard_similarities) => {
                println!("Jaccard Similarities for a BFS distance of {}:", bfs_distance);
                for ((node1, node2), similarity) in jaccard_similarities.iter() {
                    println!("Friends {} and {} have a similarity score of {}", node1, node2, similarity);
                }
                
                // Perform statistical analysis
                println!();
                println!();
                println!("Performing Statistical Analysis:");
                // Find mean similarity
                let mean_similarity = find_mean_similarity(&jaccard_similarities);
                println!("Mean Similarity Value: {}", mean_similarity);

                // Find maximum similarity and corresponding nodes
                if let Some((max_nodes, max_similarity)) = find_max_similarity(&jaccard_similarities) {
                    println!("The most similar set of friends are {:?} with a similarity of {}", max_nodes, max_similarity);
                } else {
                    println!("No maximum similarity found.");
                }

                // Find percentage of nodes above threshold
                println!();
                println!("Percentage of Friends with Certain Similarities:");
                for threshold in (1..=10).map(|x| x as f64 / 10.0) {
                    let percentage = find_percentage_above_threshold(&jaccard_similarities, threshold);
                    println!("Percentage of friends that have a similarity of {:.1} are: {:.5}%", threshold, percentage);
                }

            }
            None => println!("Failed to compute Jaccard similarity."),
        }
    }
}



//TESTS


    #[test]
    fn test_read_graph() {
        let filename = "test.txt";
        let graph = read_graph(filename).expect("Failed to read graph file.");
        assert_eq!(graph.num_nodes(), 4); // I made a txt file with 4 nodes
    }

    
    #[test]
    fn test_compute_jaccard_similarity() {
        let filename = "test.txt"; // Use a test graph file
        let bfs_distance = 2; // Set BFS distance for the test
        let similarities = compute_jaccard_similarity(filename, bfs_distance).expect("Failed to compute Jaccard similarity.");
        assert_eq!(similarities[&(0, 3)], 0.0); // Example assertion to check based on nodes in the test.txt
    }

    #[test]
    fn test_calculate_paths() {
        let filename = "test.txt"; 
        let graph = read_graph(filename).expect("Failed to read test graph");
        let paths = calculate_paths(&graph);
        println!("Calculated shortest paths: {:?}", paths);
    
        // Asserting the number of nodes in the graph
        assert_eq!(paths.len(), 4); 
        // Asserting
        assert_eq!(paths[&0][&1], 1); // Expected shortest path from node 0 to node 1 is 1
        assert_eq!(paths[&0][&3], 2); // Expected shortest path from node 0 to node 3 is 2
    }
    
    #[test]
    fn test_stats() {
        let bfs_distance = 1; // Set the BFS distance you want to compute Jaccard similarity for
        let filename = "test.txt"; 
        let graph = read_graph(filename).expect("Failed to read test graph");    
        match compute_jaccard_similarity(filename, bfs_distance) {
            Some(jaccard_similarities) => {
                // Checking mean similarity
                let mean_similarity = find_mean_similarity(&jaccard_similarities);
                let expected_mean_similarity = 0.16666666666666666; // Expected value computed manually
                assert_eq!(mean_similarity, expected_mean_similarity);
                // Checking max
                if let Some((max_nodes, max_similarity)) = find_max_similarity(&jaccard_similarities) {
                    let expected_max_similarity = 0.5; // Expected value computed manually
                    assert_eq!(max_similarity, expected_max_similarity);
                }
                // Check percentage
                let threshold = 0.2;
                let percentage = find_percentage_above_threshold(&jaccard_similarities, threshold);
                let expected_percentage = 33.33333333333333; // Expected value computed manually
                assert_eq!(percentage, expected_percentage);
            
            }
            None => println!("Failed to compute Jaccard similarity."),
        }
    }
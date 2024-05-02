use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

pub fn find_mean_similarity(similarities: &HashMap<(usize, usize), f64>) -> f64 {
    // Calculate the sum of all similarity values in the HashMap
    let sum: f64 = similarities.values().sum();
    let count = similarities.len() as f64; // Get the number of similarity pairs
    sum / count // divide to get mean
}

pub fn find_max_similarity(similarities: &HashMap<(usize, usize), f64>) -> Option<((usize, usize), f64)> {
    // Find the maximum similarity and its corresponding nodes in the HashMap
    similarities.iter().max_by(|(_, &sim1), (_, &sim2)| sim1.partial_cmp(&sim2).unwrap()).map(|(&nodes, &sim)| (nodes, sim))
}

pub fn find_percentage_above_threshold(similarities: &HashMap<(usize, usize), f64>, threshold: f64) -> f64 {
    // Count the number of similarities above the threshold
    let above_percentage_count = similarities.values().filter(|&&similarity| similarity > threshold).count() as f64;
    let total_nodes = similarities.len() as f64; // Get the total number of similarity pairs
    (above_percentage_count / total_nodes) * 100.0 // calculate percentage
}
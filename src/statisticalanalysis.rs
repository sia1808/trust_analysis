use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

pub fn find_mean_similarity(similarities: &HashMap<(usize, usize), f64>) -> f64 {
    let sum: f64 = similarities.values().sum();
    let count = similarities.len() as f64;
    sum / count
}

pub fn find_max_similarity(similarities: &HashMap<(usize, usize), f64>) -> Option<((usize, usize), f64)> {
    similarities.iter().max_by(|(_, &sim1), (_, &sim2)| sim1.partial_cmp(&sim2).unwrap()).map(|(&nodes, &sim)| (nodes, sim))
}

pub fn find_percentage_above_threshold(similarities: &HashMap<(usize, usize), f64>, threshold: f64) -> f64 {
    let above_percentage_count = similarities.values().filter(|&&similarity| similarity > threshold).count() as f64;
    let total_nodes = similarities.len() as f64;
    (above_percentage_count / total_nodes) * 100.0
}
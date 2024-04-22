mod graph;
use crate::graph::Graph;
use Graph::read_graph;

fn main() {
    let filename = "trust_data.txt";
    let graph = Graph::read_graph(filename);
}
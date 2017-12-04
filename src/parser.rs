use std::fs::File;
use std::process;
use std::str::FromStr;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

use petgraph::{
    Graph
};

#[allow(dead_code)]
fn parse_graph(file_name: &str) -> Graph<u32, ()> {
    let mut graph = Graph::<u32, ()>::new();
    let file = File::open(Path::new(file_name));

    let buffer;
    match file {
        Err(_) => { process::exit(1) }
        Ok(f) => { buffer = BufReader::new(f) }
    }

    let mut node_count = 0;
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for line in buffer.lines() {
        let line_text = line.unwrap();
        match line_text.find(" ") {
            None => process::exit(1),
            Some(index) => {
                let node = u32::from_str(&line_text[..index]).unwrap();
                let edge = u32::from_str(&line_text[index + 1..line_text.len() - 1]).unwrap();
                if node_count < node {
                    node_count += 1;
                    nodes.push(graph.add_node(node));
                }
                edges.push((node - 1, edge - 1));
            }
        }
    }
    for edge in edges {
        let (to, from) = edge;
        graph.add_edge(nodes[to as usize], nodes[from as usize], ());
   }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_parse() {
        let path = "/home/acalix/Repos/public/algorithm_study/data/SCC.txt";
        let graph = parse_graph(path);
        assert!(graph.is_directed());
        assert_eq!(graph.node_count(), 875714);
        assert_eq!(graph.edge_count(), 5105043);
    }
}

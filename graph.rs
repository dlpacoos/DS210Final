use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Graph {
    pub edges: HashMap<u32, Vec<u32>>, // node_id, Vec<neighbor_id>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: u32, b: u32) {
        self.edges.entry(a).or_insert_with(Vec::new).push(b);
        self.edges.entry(b).or_insert_with(Vec::new).push(a);
    }

    // Method to get the neighbors of a node
    pub fn neighbors(&self, id: u32) -> Vec<u32> {
        self.edges.get(&id).cloned().unwrap_or_else(Vec::new)
    }

    pub fn from_file(path: &Path) -> io::Result<Graph> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut graph = Graph::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let a = parts[0].parse::<u32>().unwrap();
                let b = parts[1].parse::<u32>().unwrap();
                graph.add_edge(a, b);
            }
        }

        Ok(graph)
    }
}

use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    // Building adjacency list
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some((node, neighbors_part)) = line.split_once(':') {
            let node_key = node.trim().to_string();
            
            let neighbors: Vec<String> = neighbors_part
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            graph.insert(node_key, neighbors);
        }
    }

    // DFS
    let start_node = "you";
    let target_node = "out";
    let mut visited = HashSet::new();

    let total_paths = count_paths(&graph, start_node, target_node, &mut visited);

    println!("Total paths from '{}' to '{}': {}", start_node, target_node, total_paths);
}

// DFS function to count paths
fn count_paths(graph: &HashMap<String, Vec<String>>, current: &str, target: &str, visited: &mut HashSet<String>) -> usize {
    if current == target {
        return 1;
    }

    if visited.contains(current) {
        return 0;
    }

    visited.insert(current.to_string());

    let mut path_count = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            path_count += count_paths(graph, neighbor, target, visited);
        }
    }

    visited.remove(current);

    path_count
}

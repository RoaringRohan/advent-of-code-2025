use std::collections::HashMap;
use std::fs;

type NodeId = usize;

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    // Encoding node names to IDs
    let mut name_to_id: HashMap<String, NodeId> = HashMap::new();
    let mut next_id = 0;

    // Helper to get or create an ID for a name
    let mut get_id = |name: &str| -> NodeId {
        if let Some(&id) = name_to_id.get(name) {
            id
        } else {
            let id = next_id;
            name_to_id.insert(name.to_string(), id);
            next_id += 1;
            id
        }
    };

    // Build adjacency list using IDs
    let mut temp_graph: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() { continue; }
        if let Some((u_str, v_str)) = line.split_once(':') {
            let u = get_id(u_str.trim());
            let neighbors: Vec<NodeId> = v_str
                .split_whitespace()
                .map(|s| get_id(s))
                .collect();
            temp_graph.insert(u, neighbors);
        }
    }

    // Convert HashMap graph to Vector for faster access
    let mut graph: Vec<Vec<NodeId>> = vec![vec![]; next_id];
    for (u, neighbors) in temp_graph {
        graph[u] = neighbors;
    }

    // Special nodes
    let svr = *name_to_id.get("svr").expect("Error: 'svr' not found in input");
    let out = *name_to_id.get("out").expect("Error: 'out' not found in input");
    let dac = *name_to_id.get("dac").unwrap_or(&usize::MAX); 
    let fft = *name_to_id.get("fft").unwrap_or(&usize::MAX);

    // Run DP with Memoization
    // Cache Key: (current_node, seen_dac, seen_fft) -> count
    let mut memo = HashMap::new();
    
    let count = count_paths_memo(
        &graph, 
        svr, 
        out, 
        dac, 
        fft, 
        false, 
        false, 
        &mut memo
    );

    println!("Total paths: {}", count);
}

fn count_paths_memo(graph: &Vec<Vec<NodeId>>, u: NodeId, target: NodeId, dac_id: NodeId, fft_id: NodeId, seen_dac: bool, seen_fft: bool, memo: &mut HashMap<(NodeId, bool, bool), usize>) -> usize {
    // Check cache
    if let Some(&count) = memo.get(&(u, seen_dac, seen_fft)) {
        return count;
    }

    // Update state
    let now_seen_dac = seen_dac || (u == dac_id);
    let now_seen_fft = seen_fft || (u == fft_id);

    // Base case: Reached target
    if u == target {
        return if now_seen_dac && now_seen_fft { 1 } else { 0 };
    }

    // Recursive step
    let mut total_paths = 0;
    for &v in &graph[u] {
        total_paths += count_paths_memo(graph, v, target, dac_id, fft_id, now_seen_dac, now_seen_fft, memo);
    }

    // Store in cache
    memo.insert((u, seen_dac, seen_fft), total_paths);
    
    total_paths
}
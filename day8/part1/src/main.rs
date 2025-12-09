use std::fs;
use std::collections::VecDeque;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Failed to read input file");
    
    let mut points: Vec<Point> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        
        if parts.len() == 3 {
            let x: i64 = parts[0].trim().parse().expect("Failed to parse x coordinate");
            let y: i64 = parts[1].trim().parse().expect("Failed to parse y coordinate");
            let z: i64 = parts[2].trim().parse().expect("Failed to parse z coordinate");

            points.push(Point { x, y, z });
        }
    }

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            
            // Squared Euclidean distance
            let dist_sq = (p1.x - p2.x).pow(2) 
                        + (p1.y - p2.y).pow(2) 
                        + (p1.z - p2.z).pow(2);
            
            edges.push((dist_sq, i, j));
        }
    }

    // Sorting edges by distance
    edges.sort_unstable_by_key(|k| k.0);

    // Breadth-First Search
    // Build adjacency list from the top 1000 edges
    let mut adj: Vec<Vec<usize>> = vec![vec![]; points.len()];
    
    // Limit to first 1000 edges
    let limit = 1000.min(edges.len());
    
    for k in 0..limit {
        let (_, u, v) = edges[k];
        adj[u].push(v);
        adj[v].push(u);
    }

    // Find connected components using BFS
    let mut visited = vec![false; points.len()];
    let mut circuit_sizes: Vec<u64> = Vec::new();

    for i in 0..points.len() {
        if !visited[i] {
            // Found a new circuit, start BFS to measure its size
            let mut size = 0;
            let mut queue = VecDeque::new();
            
            visited[i] = true;
            queue.push_back(i);

            while let Some(curr) = queue.pop_front() {
                size += 1;
                for &neighbor in &adj[curr] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
            circuit_sizes.push(size);
        }
    }

    // Multiply the sizes of the 3 largest circuits
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a)); // Sort Descending

    // Calculate the product of the top 3 sizes
    let answer: u64 = circuit_sizes.iter().take(3).product();

    println!("Found {} circuits.", circuit_sizes.len());
    println!("Top circuit sizes: {:?}", &circuit_sizes[0..3.min(circuit_sizes.len())]);
    println!("Sizes of three largest circuits multiplied: {}", answer);
}

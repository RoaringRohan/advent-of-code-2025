use std::fs;

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

    // Calculate all pairwise squared distances and store edges
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

    // Union-Find
    let mut parent: Vec<usize> = (0..points.len()).collect();
    let mut components = points.len();

    fn find(p: &mut Vec<usize>, mut i: usize) -> usize {
        while i != p[i] {
            p[i] = p[p[i]];
            i = p[i];
        }
        i
    }
    
    for (_dist, u, v) in edges {
        let root_u = find(&mut parent, u);
        let root_v = find(&mut parent, v);

        if root_u != root_v {
            // Union the components
            parent[root_u] = root_v;
            components -= 1;

            // Check if all points are now connected
            if components == 1 {
                let answer = points[u].x * points[v].x;
                println!("Product of X coordinates of last connection: {}", answer);
                return;
            }
        }
    }
}

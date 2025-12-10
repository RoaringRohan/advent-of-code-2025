use std::fs;

// Represents a line segment between two Red tiles (Green tiles)
struct Edge {
    p1: (i64, i64),
    p2: (i64, i64),
}

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Failed to read the file");

    let mut points: Vec<(i64, i64)> = Vec::new();

    // Parse input points and store them
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().parse().unwrap();
        let y: i64 = parts.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    // Calculate edges (including the wrap-around from last to first)
    let mut edges = Vec::new();
    if !points.is_empty() {
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()]; // Wrap around to 0 for the last point
            edges.push(Edge { p1, p2 });
        }
    }

    // Try all pairs of red tiles to form rectangles
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            // Rectangle must have nonzero width & height
            if x1 == x2 || y1 == y2 {
                continue;
            }

            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;
            let area = width * height;

            // Performing geometric check if current rectangle is bigger than max area found so far
            if area > max_area {
                if is_valid_rectangle(x1, y1, x2, y2, &edges) {
                    max_area = area;
                }
            }
        }
    }

    println!("Largest valid rectangle area: {}", max_area);
}

// Checks if the rectangle defined by (x1, y1) and (x2, y2) is valid
fn is_valid_rectangle(x1: i64, y1: i64, x2: i64, y2: i64, edges: &Vec<Edge>) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    // Checking edges against the rectangle
    for edge in edges {
        let ex1 = edge.p1.0.min(edge.p2.0);
        let ex2 = edge.p1.0.max(edge.p2.0);
        let ey1 = edge.p1.1.min(edge.p2.1);
        let ey2 = edge.p1.1.max(edge.p2.1);

        let is_horiz = edge.p1.1 == edge.p2.1;

        // Checking horizontal edges
        if is_horiz {
            // Horizontal edge y must be inside rect y range
            if edge.p1.1 > min_y && edge.p1.1 < max_y {
                // And x ranges must overlap
                if min_x.max(ex1) < max_x.min(ex2) {
                    return false; 
                }
            }
        // Checking vertical edges
        } else {
            // Vertical edge x must be inside rect x range
            if edge.p1.0 > min_x && edge.p1.0 < max_x {
                // And y ranges must overlap
                if min_y.max(ey1) < max_y.min(ey2) {
                    return false;
                }
            }
        }
    }

    // Check if the center point of the rectangle is inside the polygon
    let test_x_doubled = 2 * min_x + 1;
    let test_y_doubled = 2 * min_y + 1;
    
    let mut intersections = 0;

    for edge in edges {
        if edge.p1.0 == edge.p2.0 {
            let vx = edge.p1.0;
            let vy_min = edge.p1.1.min(edge.p2.1);
            let vy_max = edge.p1.1.max(edge.p2.1);

            // Double the edge coordinates for comparison
            let vx_doubled = 2 * vx;
            let vy_min_doubled = 2 * vy_min;
            let vy_max_doubled = 2 * vy_max;

            // The edge must be to the right of our test point
            if vx_doubled > test_x_doubled {
                // The ray y must fall within the edge's y range
                if test_y_doubled > vy_min_doubled && test_y_doubled < vy_max_doubled {
                    intersections += 1;
                }
            }
        }
    }

    // If intersections is Odd = Inside and if Even = Outside
    intersections % 2 != 0
}
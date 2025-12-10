use std::fs;

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Failed to read the file");

    // Store x, y coordinates of red tiles
    let mut points: Vec<(i64, i64)> = Vec::new();

    // Go through each line in the file
    for line in content.lines() {
        let line = line.trim();

        // Parse x, y coordinates
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().parse().unwrap();
        let y: i64 = parts.next().unwrap().parse().unwrap();

        // Store the point
        points.push((x, y));
    }

    let mut max_area = 0;

    // Try all pairs of red tiles
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            // Rectangle must have nonzero width & height
            if x1 == x2 || y1 == y2 {
                continue;
            }

            // Calculate width and height
            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;

            // Calculate area
            let area = width * height;

            // Update max area
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Largest rectangle area: {}", max_area);
}

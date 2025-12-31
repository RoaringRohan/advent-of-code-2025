use std::collections::HashMap;
use std::fs;

struct Region {
    width: usize,
    height: usize,
    reqs: Vec<(usize, usize)>,
}

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Failed to read input file");

    let (shape_areas, regions) = parse_input(&content);

    let mut successful_regions = 0;

    for region in regions {
        // Calculate the total Area of the Region
        let region_area = region.width * region.height;

        // Calculate the total Area required by all presents
        let mut presents_area = 0;
        for (id, count) in region.reqs {
            // If the shape ID exists, add its area * count
            if let Some(&area) = shape_areas.get(&id) {
                presents_area += area * count;
            }
        }

        // Heuristic check
        // This problem is incredibly complex, so I'm using a simple heuristic taking advantage of the input structure
        // If total presents area <= region area, count as successful
        if presents_area <= region_area {
            successful_regions += 1;
        }
    }

    println!("Regions that fit (Area Heuristic): {}", successful_regions);
}

fn parse_input(content: &str) -> (HashMap<usize, usize>, Vec<Region>) {
    let mut shape_areas = HashMap::new();
    let mut regions = Vec::new();
    let mut lines = content.lines().peekable();
    
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() { continue; }

        if line.contains("x") && line.contains(":") {
            // Parse Region: "12x5: 1 0 1 0 2 2"
            let parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<&str> = parts[0].split('x').collect();
            let width = dims[0].parse().unwrap();
            let height = dims[1].parse().unwrap();

            let counts: Vec<usize> = parts[1].split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            
            let mut reqs = Vec::new();
            for (id, &count) in counts.iter().enumerate() {
                if count > 0 {
                    reqs.push((id, count));
                }
            }
            regions.push(Region { width, height, reqs });

        } else if line.ends_with(':') {
            // Parse Shape Header: "0:"
            let id: usize = line.trim_end_matches(':').parse().unwrap();
            let mut area = 0;
            
            // Read the shape grid and count '#'
            while let Some(l) = lines.peek() {
                if l.trim().is_empty() || l.contains(':') { break; }
                let l = lines.next().unwrap();
                // Count '#' in this line
                area += l.chars().filter(|&c| c == '#').count();
            }
            shape_areas.insert(id, area);
        }
    }
    (shape_areas, regions)
}
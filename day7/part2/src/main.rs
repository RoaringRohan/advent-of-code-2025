use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    // Parse content into a grid
    let grid: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut current_beams: HashMap<usize, u64> = HashMap::new();
    
    // Finding S
    for (x, &char) in grid[0].iter().enumerate() {
        if char == 'S' {
            current_beams.insert(x, 1);
            break;
        }
    }

    let mut timelines: u64 = 0;

    // Iterate through each row of the grid
    for y in 0..height - 1 {
        let mut next_beams: HashMap<usize, u64> = HashMap::new();

        // Process each beam in the current row
        for (&x, &count) in &current_beams {
            // Check the row directly below the current beam
            let next_cell = grid[y + 1][x];

            match next_cell {
                '.' => {
                    // Beam passes through empty space, column x stays active
                    *next_beams.entry(x).or_insert(0) += count;
                }
                '^' => {
                    // Split left
                    if x > 0 {
                        *next_beams.entry(x - 1).or_insert(0) += count;
                    }
                    // Falls off grid if at left edge
                    else {
                        timelines += count;
                    }
                    
                    // SPlit right
                    if x + 1 < width {
                        *next_beams.entry(x + 1).or_insert(0) += count;
                    }
                    // Falls off grid if at right edge
                    else {
                        timelines += count;
                    }
                }
                _ => {
                }
            }
        }

        // Update the state for the next iteration
        current_beams = next_beams;

        // If no beams are left, we can stop early
        if current_beams.is_empty() {
            break;
        }
    }

    // Add remaining beams at the bottom row to timelines
    let timelines_at_bottom: u64 = current_beams.values().sum();
    let total = timelines + timelines_at_bottom;

    println!("Total splits: {}", total);
}
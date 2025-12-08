use std::fs;
use std::collections::HashSet;

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

    let mut current_beams: HashSet<usize> = HashSet::new();
    
    // Finding S
    for (x, &char) in grid[0].iter().enumerate() {
        if char == 'S' {
            current_beams.insert(x);
            break;
        }
    }

    let mut split_count = 0;

    // Iterate through each row of the grid
    for y in 0..height - 1 {
        let mut next_beams: HashSet<usize> = HashSet::new();

        // Process each beam in the current row
        for &x in &current_beams {
            // Check the row directly below the current beam
            let next_cell = grid[y + 1][x];

            match next_cell {
                '.' => {
                    // Beam passes through empty space, column x stays active
                    next_beams.insert(x);
                }
                '^' => {
                    // Found a splitter
                    split_count += 1;

                    // Beam splits into left and right
                    if x > 0 {
                        next_beams.insert(x - 1);
                    }
                    if x + 1 < width {
                        next_beams.insert(x + 1);
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

    println!("Total splits: {}", split_count);
}
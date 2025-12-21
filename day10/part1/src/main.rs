use std::fs;

#[derive(Debug)]
struct Machine {
    target: Vec<u8>,
    buttons: Vec<Vec<usize>>,
}

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Failed to read the file");

    let mut total = 0;

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // [..#.]
        let diagram_str = parts[0];
        // ..#.
        let clean_diagram = &diagram_str[1..diagram_str.len()-1];

        // Converting diagram to 0s and 1s representation
        let target: Vec<u8> = clean_diagram.chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Unexpected character in diagram"),
            })
            .collect();

        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for part in &parts[1..] {
            // Ignore joltage input
            if part.starts_with('{') {
                break;
            }

            // Turns (1,3) into 1,3
            let clean_part = part.trim_matches(|c| c == '(' || c == ')');

            // Split by comma and parse numbers
            let button_indices: Vec<usize> = clean_part
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            buttons.push(button_indices);
        }

        let machine = Machine { target, buttons };

        if let Some(solution) = solve_machine(&machine) {
            total += solution;
        }
    }
    println!("Total minimized presses for all machines: {}", total);
}

fn solve_machine(machine: &Machine) -> Option<usize> {
    let num_rows = machine.target.len();
    let num_cols = machine.buttons.len();

    // Create matrix of size [rows][cols + 1] initialized to 0
    let mut matrix = vec![vec![0u8; num_cols + 1]; num_rows];

    // Creating setup for linear algebra equation in matrix form
    // Fill columns based on buttons (each column is the output of what the lights look like once that button is pressed)
    for (col_idx, light_indices) in machine.buttons.iter().enumerate() {
        for &row_idx in light_indices {
            if row_idx < num_rows {
                matrix[row_idx][col_idx] = 1;
            }
        }
    }

    // Last column of the matrix is the target configuration (based on schematic)
    for (row_idx, &val) in machine.target.iter().enumerate() {
        matrix[row_idx][num_cols] = val;
    }

    // Now to find minimum solution, we need to perform Gaussian elimination
    // Performing Gaussian elimination
    let mut pivot_row = 0;
    let mut pivots = Vec::new();

    for col in 0..num_cols {
        if pivot_row >= num_rows {
            break;
        }

        // Find a row at or below 'pivot_row' that has a 1 in this column
        let mut found_row = None;
        for r in pivot_row..num_rows {
            if matrix[r][col] == 1 {
                found_row = Some(r);
                break;
            }
        }

        if let Some(r_idx) = found_row {
            // Swap that row up to the pivot position
            matrix.swap(pivot_row, r_idx);

            // Eliminate all other rows in this column (make them 0)
            for r in 0..num_rows {
                if r != pivot_row && matrix[r][col] == 1 {
                    // XOR the entire row 'r' with the 'pivot_row'
                    for c in 0..=num_cols {
                        matrix[r][c] ^= matrix[pivot_row][c];
                    }
                }
            }

            pivots.push((pivot_row, col));
            pivot_row += 1;
        }
    }

    // Checking for inconsistency (0 = 1 rows)
    for r in 0..num_rows {
        let is_row_zero = matrix[r][0..num_cols].iter().all(|&x| x == 0);
        if is_row_zero && matrix[r][num_cols] == 1 {
            return None; // Impossible to solve
        }
    }

    // Finding the free variables
    // Collect column indices that have pivots
    let pivot_cols: Vec<usize> = pivots.iter().map(|&(_, c)| c).collect();
    // Any column not in pivot_cols is free
    let free_cols: Vec<usize> = (0..num_cols)
        .filter(|c| !pivot_cols.contains(c))
        .collect();

    let mut min_presses = usize::MAX;

    // Brute force over all combinations of free variables
    let num_free = free_cols.len();
    // 1 << num_free is 2^num_free, (E.g. if 2 free vars, loop 0..4)
    for i in 0..(1 << num_free) {
        let mut solution = vec![0u8; num_cols];

        // Set the free variables based on the bits of 'i'
        for (bit_idx, &col_idx) in free_cols.iter().enumerate() {
            if (i >> bit_idx) & 1 == 1 {
                solution[col_idx] = 1;
            }
        }

        // Solve for pivot variables
        for &(r, c) in pivots.iter().rev() {
            let mut row_val = matrix[r][num_cols];
            
            // Subtract (XOR) the values of valid buttons to the right
            for k in (c + 1)..num_cols {
                if matrix[r][k] == 1 {
                    row_val ^= solution[k];
                }
            }
            solution[c] = row_val;
        }

        // Count total presses
        let current_presses: usize = solution.iter().map(|&x| x as usize).sum();
        if current_presses < min_presses {
            min_presses = current_presses;
        }
    }

    if min_presses == usize::MAX {
        None
    } else {
        Some(min_presses)
    }
}
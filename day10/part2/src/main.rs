use std::fs;

#[derive(Debug)]
struct Machine {
    target: Vec<u8>,
    joltage: Vec<i64>,
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
        let mut joltage: Vec<i64> = Vec::new();

        for part in &parts[1..] {
            if part.starts_with('{') {
                let clean_part = part.trim_matches(|c| c == '{' || c == '}');
                joltage = clean_part
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                continue; 
            }

            // Turns (1,3) into 1,3
            let clean_part = part.trim_matches(|c| c == '(' || c == ')');
            let button_indices: Vec<usize> = clean_part
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            buttons.push(button_indices);
        }

        let machine = Machine { target, joltage, buttons };

        if let Some(solution) = solve_machine(&machine) {
            total += solution;
        }
    }
    println!("Total minimized presses for all machines: {}", total);
}

fn solve_machine(machine: &Machine) -> Option<i64> {
    let num_rows = machine.joltage.len();
    let num_cols = machine.buttons.len();

    // Setup matrix
    let mut matrix = vec![vec![0.0; num_cols + 1]; num_rows];
    for (col_idx, light_indices) in machine.buttons.iter().enumerate() {
        for &row_idx in light_indices {
            if row_idx < num_rows {
                matrix[row_idx][col_idx] = 1.0;
            }
        }
    }
    for (row_idx, &val) in machine.joltage.iter().enumerate() {
        matrix[row_idx][num_cols] = val as f64;
    }

    // Gaussian Elimination
    let mut pivot_row = 0;
    let mut pivots = Vec::new();
    let epsilon = 1e-4;

    for col in 0..num_cols {
        if pivot_row >= num_rows { break; }

        let mut pivot_idx = None;
        for r in pivot_row..num_rows {
            if matrix[r][col].abs() > epsilon {
                pivot_idx = Some(r);
                break;
            }
        }

        if let Some(r) = pivot_idx {
            matrix.swap(pivot_row, r);
            let divisor = matrix[pivot_row][col];
            for c in 0..=num_cols {
                matrix[pivot_row][c] /= divisor;
            }
            for r_target in 0..num_rows {
                if r_target != pivot_row {
                    let factor = matrix[r_target][col];
                    if factor.abs() > epsilon {
                        for c in 0..=num_cols {
                            matrix[r_target][c] -= factor * matrix[pivot_row][c];
                        }
                    }
                }
            }
            pivots.push((pivot_row, col));
            pivot_row += 1;
        }
    }

    // Identify free variables
    let pivot_cols: Vec<usize> = pivots.iter().map(|&(_, c)| c).collect();
    let free_cols: Vec<usize> = (0..num_cols).filter(|c| !pivot_cols.contains(c)).collect();

    // Recursive search for integer solutions
    let mut min_total = i64::MAX;
    let mut solution = vec![0.0; num_cols];


    // Recursive function to search for valid assignments    
    fn search(
        idx: usize, 
        free_cols: &Vec<usize>, 
        solution: &mut Vec<f64>, 
        matrix: &Vec<Vec<f64>>, 
        pivots: &Vec<(usize, usize)>,
        min_total: &mut i64
    ) {
        if idx == free_cols.len() {
            // Base case: All free variables assigned. Calculate pivots.
            let mut current_valid = true;
            
            // Calculate pivots from bottom up
            for &(r, c) in pivots.iter() {
                let mut val = matrix[r][matrix[0].len() - 1];
                for free_c in free_cols {
                    val -= matrix[r][*free_c] * solution[*free_c];
                }
                
                // Check if integer and non-negative
                let rounded = val.round();
                if (val - rounded).abs() > 1e-4 || rounded < -0.1 {
                    current_valid = false;
                    break;
                }
                solution[c] = rounded;
            }

            if current_valid {
                let total: i64 = solution.iter().map(|&x| x as i64).sum();
                if total < *min_total {
                    *min_total = total;
                }
            }
            return;
        }

        // Recursive step: Try values 0..=250 for the current free variable
        let col = free_cols[idx];
        for val in 0..=250 {
            solution[col] = val as f64;
            search(idx + 1, free_cols, solution, matrix, pivots, min_total);
        }
    }

    search(0, &free_cols, &mut solution, &matrix, &pivots, &mut min_total);

    if min_total == i64::MAX { None } else { Some(min_total) }
}
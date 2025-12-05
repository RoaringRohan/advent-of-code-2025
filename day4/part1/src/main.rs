use std::fs;


fn main() {
    // Save entire txt file as a single string
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let mut accessible_count = 0;

    let mut matrix: Vec<Vec<char>> = Vec::new();

    // Make matrix of chars from input
    for line in content.lines() {
        let chars: Vec<char> = line
            .chars()
            .collect();

        matrix.push(chars);
    }

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '@' {
                let access = can_access(&matrix, row, col);
                if access {
                    accessible_count += 1;
                }
            }
        }
    }
    println!("Number of rolls that can be accessed: {}", accessible_count);
}

fn can_access(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut found_rolls = 0;

    // Checking 8 directions
    // Up
    if row > 0 {
        // Up center
        if matrix[row-1][col] == '@' {
            found_rolls += 1;
        }

        // Up left
        if col > 0 {
            if matrix[row-1][col-1] == '@' {
                found_rolls += 1;
            }
        }

        // Up right
        if col + 1 < matrix[0].len() {
            if matrix[row-1][col+1] == '@' {
                found_rolls += 1;
            }
        }
    }

    // Down
    if row + 1 < matrix.len() {
        // Down center
        if matrix[row+1][col] == '@' {
            found_rolls += 1;
        }

        // Down left
        if col > 0 {
            if matrix[row+1][col-1] == '@' {
                found_rolls += 1;
            }
        }

        // Down right
        if col + 1 < matrix[0].len() {
            if matrix[row+1][col+1] == '@' {
                found_rolls += 1;
            }
        }
    }

    // Middle left
    if col > 0 {
        if matrix[row][col-1] == '@' {
            found_rolls += 1;
        }
    }

    // Middle right
    if col + 1 < matrix[0].len() {
        if matrix[row][col+1] == '@' {
            found_rolls += 1;
        }
    }

    if found_rolls > 3 {
        return false;
    }
    else {
        return true;
    }
}

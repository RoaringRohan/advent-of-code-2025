use std::fs;

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.lines().collect();

    // Finding longest line length
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut grid: Vec<Vec<char>> = Vec::new();

    // Making sure every row will have the same length by adding spaces to shorter-length rows
    for line in lines {
        let mut row: Vec<char> = line.chars().collect();
        // Pad with spaces to the right if shorter
        while row.len() < max_len {
            row.push(' ');
        }
        grid.push(row);
    }

    // Find operation
    let last_row_idx = grid.len() - 1;

    let mut grand_total: i64 = 0;

    // Identify each block of cephalopod questions
    let mut block_ranges: Vec<(usize, usize)> = Vec::new();
    let mut start_col = 0;
    
    for x in 0..max_len {
        // Check if column 'x' is a separator (all spaces in the number rows)
        let is_separator = (0..last_row_idx).all(|y| grid[y][x] == ' ');

        // If it is separating a block, then add the range of the block to a vector
        if is_separator {
            if x > start_col {
                block_ranges.push((start_col, x));
            }
            start_col = x + 1;
        }
    }
    // Push the final block
    if start_col < max_len {
        block_ranges.push((start_col, max_len));
    }

    // Process each block (set of ranges) to compute mathematical operations
    for (start, end) in block_ranges {
        // Find the operator for this block
        let mut op_char = '+';
        for x in start..end {
            let c = grid[last_row_idx][x];
            if c == '+' || c == '-' || c == '*' || c == '/' {
                op_char = c;
                break;
            }
        }

        // Initialize if using multiplication, division to 1 else 0
        let mut problem_total: i64 = if op_char == '*' || op_char == '/' { 1 } else { 0 };

        // Read columns right-to-left (left to right but in reverse)
        for x in (start..end).rev() {
            let mut digits = String::new();

            // Read top-to-bottom (excluding the operator row)
            for y in 0..last_row_idx {
                let c = grid[y][x];
                if c.is_digit(10) {
                    digits.push(c);
                }
            }

            if !digits.is_empty() {
                let num: i64 = digits.parse().unwrap();
                
                match op_char {
                    '+' => problem_total += num,
                    '-' => problem_total -= num,
                    '*' => problem_total *= num,
                    '/' => problem_total /= num,
                    _ => {},
                }
            }
        }
        grand_total += problem_total;
    }

    println!("Grand Total: {}", grand_total);
}
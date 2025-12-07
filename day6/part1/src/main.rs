use std::fs;

fn main() {
    // Save entire txt file as a single string
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut operations: Vec<String> = Vec::new();

    // Split the string into lines and then into numbers and operations
    for line in content.lines() {
        let mut row: Vec<u32> = Vec::new();

        // Split each line by whitespace
        for str in line.split_whitespace() {
            // If str is an operation, add it to operations vector
            if str == "+" || str == "-" || str == "*" || str == "/" {
                operations.push(str.to_string());
            }

            // Else, parse it as a number and add it to the current row
            else {
                let num: u32 = str.parse().expect("Failed to parse number");
                row.push(num);
            }
        }

        // Add the current row to the matrix
        if !row.is_empty() {
            matrix.push(row);
        }
    }

    let mut sum: i64 = 0;

    // Perform operations column-wise
    for i in 0..operations.len() {
        let current_operation = &operations[i];

        let mut equals = if current_operation == "*" || current_operation == "/" {
            1
        } else {
            0
        };

        // Apply the operation to each element in the current column of matrix
        for j in 0..matrix.len() {
            match current_operation.as_str() {
                "+" => equals += matrix[j][i] as i64,
                "-" => equals -= matrix[j][i] as i64,
                "*" => equals *= matrix[j][i] as i64,
                "/" => equals /= matrix[j][i] as i64,
                _ => (),
            }
        }

        // Add the result of the current column to the sum
        sum += equals;
    }

    println!("Final result: {}", sum);
}

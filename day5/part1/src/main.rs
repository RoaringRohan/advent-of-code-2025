use std::fs;

fn main() {
    // Save entire txt file as a single string
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let mut fresh_ingredient_count: u64 = 0;

    let mut fresh_ingredients_ranges: Vec<(u64, u64)> = Vec::new();

    for line in content.lines() {
        // Check if the line contains a range
        if let Some((start_str, end_str)) = line.split_once('-') {
            // Parse strings into u64 integers
            let start: u64 = start_str.parse().expect("Failed to parse start number");
            let end: u64 = end_str.parse().expect("Failed to parse end number");

            // Store the range as a tuple
            fresh_ingredients_ranges.push((start, end));
        }
        // Otherwise, check if the line contains an available ingredient ID
        else if !line.trim().is_empty() {
            // Parse the available ingredient ID
            let available_ingredient_id: u64 = line.parse().expect("Failed to parse available ingredient ID");
            
            // Check if the available ingredient ID falls within any of the fresh ingredient ranges
            if fresh_ingredients_ranges.iter().any(|&(start, end)| available_ingredient_id >= start && available_ingredient_id <= end) {
                fresh_ingredient_count += 1;
            }
        }
    }
    println!("Number of fresh ingredients available: {}", fresh_ingredient_count);
}

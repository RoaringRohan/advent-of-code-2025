use std::fs;
use std::cmp;

fn main() {
    // Save entire txt file as a single string
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");

    let mut fresh_ingredient_count: u64 = 0;

    let mut fresh_ingredients_ranges: Vec<(u64, u64)> = Vec::new();

    for line in content.lines() {
        // Check if the line contains a range
        if let Some((start_str, end_str)) = line.split_once('-') {
            let start: u64 = start_str.parse().expect("Failed to parse start number");
            let end: u64 = end_str.parse().expect("Failed to parse end number");
        
            // Store the range as a tuple
            fresh_ingredients_ranges.push((start, end));
        }
    }

    // Sort ranges by their starting values
    fresh_ingredients_ranges.sort_unstable();

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    let (mut current_start, mut current_end) = fresh_ingredients_ranges[0];

    // Merge overlapping or contiguous ranges
    for &(next_start, next_end) in fresh_ingredients_ranges.iter().skip(1) {
        // Check if the next range overlaps or is contiguous with the current range
        if next_start <= current_end + 1 {
            current_end = cmp::max(current_end, next_end);
        } 
        // No overlap, add the current range to merged_ranges and start a new range
        else {
            merged_ranges.push((current_start, current_end));
            current_start = next_start;
            current_end = next_end;
        }
    }

    // Add the last range
    merged_ranges.push((current_start, current_end));

    // Calculate the total number of fresh ingredients available
    for (start, end) in merged_ranges {
        fresh_ingredient_count += end - start + 1;
    }

    println!("Number of fresh ingredients available: {}", fresh_ingredient_count);
}

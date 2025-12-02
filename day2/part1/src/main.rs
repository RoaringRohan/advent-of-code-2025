use std::fs;

fn main() {
    // Save entire txt file as a single string
    let content = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut sum: u64 = 0;

    // Split and iterate through each range in the string
    for range_raw in content.trim().split(',') {
        // split_once returns a tuple like Some(("655", "1102"))
        if let Some((start_str, end_str)) = range_raw.split_once('-') {
            // Parse strings into u64 integers
            let start: u64 = start_str.parse().expect("Failed to parse start number");
            let end: u64 = end_str.parse().expect("Failed to parse end number");

            // 'start..=end' includes both the start and the end number
            for num in start..=end {
                let string = num.to_string();
                let length = string.len();
                if length % 2 == 0 {
                    let mid = length / 2;
                    let (first_half, second_half) = string.split_at(mid);
                    if first_half == second_half {
                        sum += num;
                    }
                }
            }
        }
    }
    println!("Sum of all invalid numbers: {}", sum);
}

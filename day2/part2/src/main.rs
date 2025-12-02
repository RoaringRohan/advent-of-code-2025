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
                // Convert to string
                let string = num.to_string();

                // Find length of string
                let length = string.len();

                // Find middle point of length
                let middle = length / 2;

                // Iterate through possible substring lengths from 1 to middle
                for sub_len in 1..=middle {
                    
                    // Check if length is divisible by sub_len
                    if length % sub_len == 0 {
                        
                        // Get the current substring
                        let substring = &string[..sub_len];
                        
                        // Find how many times it would need to repeat
                        let repetitions = length / sub_len;
                        
                        // Check if repeating the substring forms the original string
                        if string == substring.repeat(repetitions) {
                            sum += num;
                            break; 
                        }
                    }
                }
            }
        }
    }
    println!("Sum of all invalid numbers: {}", sum);
}

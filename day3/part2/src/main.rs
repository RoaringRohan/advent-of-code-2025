use std::fs;

fn main() {
    // Save entire txt file as a single string
    let content = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    let mut sum: u64 = 0;

    for range_raw in content.lines() {
        sum += bank_joltage_maxxing(range_raw);
    }

    println!("Sum of all maxxed bank joltages: {}", sum);
}

fn bank_joltage_maxxing(input: &str) -> u64 {
    let digits: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut start_index = 0;
    let mut result: u64 = 0;
    
    for i in 0..12 {
        let needed_after = 11 - i;
        let end_limit = digits.len() - needed_after;

        let window = &digits[start_index..end_limit];
        let max_val = window.iter().max().unwrap();

        let offset = window.iter().position(|d| d == max_val).unwrap();

        result = result * 10 + (*max_val as u64);
        start_index += offset + 1;
    }
    
    result
}
use std::fs;

fn main() {
    // Save entire txt file as a single string
    let content = fs::read_to_string("../input.txt")
        .expect("Should have been able to read the file");
    
    let mut sum: u32 = 0;

    for range_raw in content.lines() {
        sum += bank_joltage_maxxing(range_raw);
    }

    println!("Sum of all maxxed bank joltages: {}", sum);
}

fn bank_joltage_maxxing(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let tens = digits[0..digits.len()-1].iter().max().unwrap();

    let tens_index = digits.iter().position(|&x| x == *tens).unwrap();

    let ones = digits[tens_index+1..].iter().max().unwrap();

    tens * 10 + ones
}
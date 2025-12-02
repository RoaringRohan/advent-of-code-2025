use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Open the "input.txt" file
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut current_pos = 50;
    let mut pointed_at_zero = 0;

    // Read each line
    for line in reader.lines() {
        let line = line?;
        
        // Get the direction
        let direction = line.chars().next().unwrap();

        // Get the number
        let rest = &line[direction.len_utf8()..];
        let num: i32 = rest.trim().parse().unwrap();

        // Update the current position based on direction
        match direction {
            'R' => {
                current_pos = (current_pos + num) % 100;
            }
            'L' => {
                current_pos = (current_pos - num) % 100;
                if current_pos < 0 {
                    current_pos += 100;
                }
            }
            _ => panic!("Invalid direction")
        }

        if current_pos == 0 {
            pointed_at_zero += 1;
        }
    }

    println!("Number of times pointed at zero: {}", pointed_at_zero);
    Ok(())
}

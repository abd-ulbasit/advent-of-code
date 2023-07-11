use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn _part_one() -> i32 {
    let file = File::open("./src/twenty_two/day1/input.txt").expect("Failed to open file");

    // Create a BufReader to read the file line by line
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut max_sum = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                // Blank line encountered, update max_sum and reset sum
                if sum > max_sum {
                    max_sum = sum;
                }
                sum = 0;
            } else {
                // Parse line into a number and add it to the sum
                if let Ok(number) = line.parse::<i32>() {
                    sum += number;
                } else {
                    eprintln!("Failed to parse line: {}", line);
                }
            }
        } else {
            eprintln!("Failed to read line");
        }
    }
    max_sum
}

pub fn _part_two() -> i32 {
    let file = File::open("./src/twenty_two/day1/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut maxes = [0; 3];

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                if sum > maxes[0] {
                    maxes[0] = sum;
                    maxes.sort();
                }
                sum = 0;
            } else {
                if let Ok(number) = line.parse::<i32>() {
                    sum += number
                }
            }
        }
    }
    let mut total = 0;
    for i in maxes {
        total += i;
    }

    total
}

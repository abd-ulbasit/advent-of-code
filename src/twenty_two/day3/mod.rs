use std::{char, collections::HashMap, fs::File, io::BufRead, io::BufReader};

pub fn _part_one() -> u32 {
    let mut priority_map: HashMap<char, u32> = HashMap::new();

    // Assign priorities for lowercase letters 'a' to 'z'
    let mut priority = 1;
    for c in 'a'..='z' {
        priority_map.insert(c, priority);
        priority += 1;
    }

    // Assign priorities for uppercase letters 'A' to 'Z'
    priority = 27;
    for c in 'A'..='Z' {
        priority_map.insert(c, priority);
        priority += 1;
    }

    // Assign priorities for lowercase letters 'a' to 'z'
    let mut sum = 0;
    let file = File::open("./src/twenty_two/day3/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("cant read line");
        let mid_point = line.len() / 2;
        let (first_half, second_half) = line.split_at(mid_point);
        let mut char: Option<char> = None;
        for char1 in first_half.chars() {
            if second_half.contains(char1) {
                char = Some(char1)
            }
        }
        if let Some(ch) = char {
            sum += priority_map.get(&ch).unwrap();
        }
    }

    sum
}

use std::{fs::File, io::BufRead, io::BufReader};

pub fn _part_one() -> u32 {
    let mut count = 0;
    let file = File::open("./src/twenty_two/day4/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let (first_range, second_range) = line.split_at(line.find(',').unwrap());
            let second_range = &second_range[1..];
            // println!("{}___{}", first_range, second_range);
            let (f_s, f_e) = first_range.split_at(first_range.find('-').unwrap());
            let (s_s, s_e) = second_range.split_at(second_range.find('-').unwrap());
            // println!("{}_{}_____{}_{}", f_s, f_e, s_s, s_e);
            let first_range =
                (f_s.parse::<i32>()).unwrap()..=(f_e.clone()[1..].parse::<i32>()).unwrap();
            let second_range =
                (s_s.parse::<i32>()).unwrap()..=(s_e.clone()[1..].parse::<i32>()).unwrap();
            if (first_range.contains(second_range.start())
                && first_range.contains(second_range.end()))
                || (second_range.contains(first_range.start())
                    && second_range.contains(first_range.end()))
            {
                count += 1;
            }
        }
    }
    count
}
pub fn _part_two() -> u32 {
    let mut count = 0;
    let file = File::open("./src/twenty_two/day4/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let (first_range, second_range) = line.split_at(line.find(',').unwrap());
            let second_range = &second_range[1..];
            // println!("{}___{}", first_range, second_range);
            let (f_s, f_e) = first_range.split_at(first_range.find('-').unwrap());
            let (s_s, s_e) = second_range.split_at(second_range.find('-').unwrap());
            // println!("{}_{}_____{}_{}", f_s, f_e, s_s, s_e);
            let first_range =
                (f_s.parse::<i32>()).unwrap()..=(f_e.clone()[1..].parse::<i32>()).unwrap();
            let second_range =
                (s_s.parse::<i32>()).unwrap()..=(s_e.clone()[1..].parse::<i32>()).unwrap();
            if first_range.contains(second_range.start())
                || first_range.contains(second_range.end())
                || second_range.contains(first_range.start())
                || second_range.contains(first_range.end())
            {
                count += 1;
            }
        }
    }
    count
}

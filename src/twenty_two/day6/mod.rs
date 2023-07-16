use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn _solution(second: bool) -> u32 {
    let file = File::open("./src/twenty_two/day6/input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let (mut l, mut r) = (0, if second { 14 } else { 4 });
    while r < input.len() {
        let mut char_set = HashSet::new();
        if !&input[l..r].chars().any(|c| !char_set.insert(c.clone())) {
            return r as u32;
        }
        l += 1;
        r += 1;
    }
    1
}

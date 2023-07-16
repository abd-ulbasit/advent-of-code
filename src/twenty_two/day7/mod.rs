use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution(_second: bool) -> i32 {
    let file = File::open("./src/twenty_two/day7/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut pwd: Vec<String> = vec![];
    let mut csize = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let words: Vec<&str> = line.split_whitespace().collect();
            match words.as_slice() {
                &["$", "cd", "/"] => pwd.clear(),
                &["$", "cd", ".."] => {
                    pwd.pop().unwrap();
                    ()
                }
                &["$", "cd", x] => pwd.push(x.to_string()),
                &["$", "ls"] | &["dir", _] => {}
                &[size, _name] => {
                    for p in prefixes(&pwd) {
                        let size_int: i32 = size.parse().unwrap_or(0);
                        *csize.entry(p).or_insert(0) += size_int;
                    }
                }
                _ => {}
            }
        }
    }
    println!("{:?}", csize);
    csize
        .values()
        .filter(|&&val| val < 100000)
        .collect::<Vec<&i32>>()
        .iter()
        .copied()
        .sum()
}

fn prefixes(pwd: &[String]) -> Vec<String> {
    let mut prefixes: Vec<String> = Vec::new();
    for i in 0..pwd.len() {
        prefixes.push(pwd[..=i].join("/"));
    }
    prefixes
}

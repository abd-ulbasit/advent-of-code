use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution(second: bool) -> Result<i32, Box<dyn Error>> {
    let file = File::open("./src/twenty_two/day10/input.txt")?;
    let reader = BufReader::new(file);
    let mut val = 1;
    let mut sum = 0;
    let mut cycle_no = {
        if second {
            0
        } else {
            1
        }
    };
    let cycle_list = [20, 60, 100, 140, 180, 220].to_vec();
    for line in reader.lines() {
        if let Ok(line) = line {
            let words: Vec<&str> = line.split_whitespace().collect();
            match words.as_slice() {
                &["addx", x] => {
                    let x = x.to_string().parse::<i32>()?;
                    // println!("addx {}", x);
                    if cycle_no % 40 == 0 {
                        println!();
                    }
                    if (val - 1..=val + 1).contains(&(cycle_no % 40)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                    cycle_no += 1;
                    if cycle_list.contains(&cycle_no) {
                        sum += val * cycle_no;
                    }
                    if cycle_no % 40 == 0 {
                        println!();
                    }
                    if (val - 1..=val + 1).contains(&(cycle_no % 40)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                    cycle_no += 1;
                    val += x;
                    if cycle_list.contains(&cycle_no) {
                        sum += val * cycle_no;
                    }
                }
                &["noop"] => {
                    // println!("Noop");
                    if cycle_no % 40 == 0 {
                        println!();
                    }
                    cycle_no += 1;
                    if (val - 1..=val + 1).contains(&(cycle_no % 40)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                    if cycle_list.contains(&cycle_no) {
                        sum += val * cycle_no;
                    }
                }
                _ => (),
            }
        }
    }

    Ok(sum)
}

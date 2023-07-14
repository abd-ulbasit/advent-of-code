use std::{fs::File, io::BufRead, io::BufReader};

pub fn solution(secod_part: bool) -> String {
    let mut tops = String::new();
    let file = File::open("./src/twenty_two/day5/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut splitter = lines.split(|line| line == "");
    let stack_input = splitter.next().unwrap();
    let num_line = stack_input.last().unwrap();
    let movement_input = splitter.next().unwrap();
    let mut stacks_vec: Vec<Vec<char>> = vec![Vec::new(); 10];
    // for line in stack_input {
    //     println!("{:?}", line);
    // }
    // println!("{}", num_line);
    for (i, ch) in num_line.chars().enumerate() {
        if ch.is_numeric() {
            let stack_no = ch.to_string().parse::<usize>().unwrap();
            for line in stack_input.iter().rev().skip(1) {
                let current_char = line.chars().nth(i).unwrap();

                if current_char != ' ' {
                    stacks_vec[stack_no].push(current_char);
                }
            }
        }
    }
    // println!("{:?}", stacks_vec);
    for movement in movement_input {
        let parts: Vec<&str> = movement.split_whitespace().collect();
        let (mut no_of_movements, mut from, mut to) = (0, 0, 0);
        if let (Some(num1), Some(num2), Some(num3)) = (parts.get(1), parts.get(3), parts.get(5)) {
            if let (Ok(value1), Ok(value2), Ok(value3)) = (
                num1.parse::<i32>(),
                num2.parse::<usize>(),
                num3.parse::<usize>(),
            ) {
                // println!("Numeric values: {}, {}, {}", value1, value2, value3);
                no_of_movements = value1;
                from = value2;
                to = value3;
            }
        }
        if !secod_part {
            for _ in 0..no_of_movements {
                let top = stacks_vec[from].pop().unwrap();
                stacks_vec[to].push(top);
            }
        } else {
            let mut temp = vec![];
            for _ in 0..no_of_movements {
                temp.push(stacks_vec[from].pop().unwrap());
            }
            while let Some(top) = temp.pop() {
                stacks_vec[to].push(top);
            }
        }
    }
    for i in stacks_vec {
        if let Some(t) = i.last() {
            tops.push(*t);
        }
    }
    tops
}

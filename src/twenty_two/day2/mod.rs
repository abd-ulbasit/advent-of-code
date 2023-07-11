use std::{collections::HashMap, fs::File, hash::Hash, io::BufRead, io::BufReader};

pub fn part_one() -> i32 {
    let file = File::open("./src/twenty_two/day2/input.txt").expect("Failed to open file");

    // Create a BufReader to read the file line by line
    let reader = BufReader::new(file);

    let mut myscore = 0;
    let moves = vec![
        ('A', 'X'),
        ('A', 'Y'),
        ('A', 'Z'),
        ('B', 'X'),
        ('B', 'Y'),
        ('B', 'Z'),
        ('C', 'X'),
        ('C', 'Y'),
        ('C', 'Z'),
    ];
    let scores = vec![4, 8, 3, 1, 5, 9, 7, 2, 6];
    let points_table: HashMap<(char, char), i32> =
        moves.into_iter().zip(scores.into_iter()).collect();
    println!("{:?}", points_table);

    // Iterate over each line in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            // if line.is_empty() {
            //     break;
            // };
            let moves: (char, char) = (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());

            myscore += points_table.get(&moves).unwrap();
            println!("{:?}", myscore);
        }
    }

    myscore
}

pub fn part_two() -> i32 {
    let file = File::open("./src/twenty_two/day2/input.txt").expect("Failed to open file");

    // Create a BufReader to read the file line by line
    let reader = BufReader::new(file);

    let mut myscore = 0;
    let messages = vec![
        ('A', 'X'),
        ('A', 'Y'),
        ('A', 'Z'),
        ('B', 'X'),
        ('B', 'Y'),
        ('B', 'Z'),
        ('C', 'X'),
        ('C', 'Y'),
        ('C', 'Z'),
    ];
    let scores = vec![4, 8, 3, 1, 5, 9, 7, 2, 6];
    let my_move = vec!['Z', 'X', 'Y', 'X', 'Y', 'Z', 'Y', 'Z', 'X'];
    let points_table: HashMap<(char, char), i32> =
        messages.iter().cloned().zip(scores.into_iter()).collect();
    // println!("{:?}", points_table);
    let my_move: HashMap<(char, char), char> =
        messages.into_iter().zip(my_move.into_iter()).collect();
    // Iterate over each line in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            // if line.is_empty() {
            //     break;
            // };
            let moves: (char, char) = (
                line.chars().nth(0).unwrap(),
                *(my_move
                    .get(&(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
                    .unwrap()),
            );

            myscore += points_table.get(&moves).unwrap();
            println!("{:?}", myscore);
        }
    }

    myscore
}

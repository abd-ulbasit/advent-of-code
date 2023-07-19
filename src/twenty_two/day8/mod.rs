use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

pub fn solution(_second: bool) -> Result<i32, Box<dyn Error>> {
    let file = File::open("./src/twenty_two/day8/input.txt")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let input = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut is_visible_grid =
        vec![vec![false; input.iter().next().unwrap().clone().len()]; input.len()];

    // println!("{:?}", input);
    let rows = input.len();
    let cols = input[0].len();
    // let mut visible_trees = 0;

    // Check visibility from the top
    let mut max_heights = vec![-1; cols];
    for j in 0..cols {
        for i in 0..rows {
            if input[i][j] > max_heights[j] {
                // visible_trees += 1;
                is_visible_grid[i][j] = true;
                max_heights[j] = input[i][j];
            }
        }
    }
    // Check visibility from the bottom
    let mut max_heights = vec![-1; cols];
    for j in 0..cols {
        for i in (0..rows).rev() {
            if input[i][j] > max_heights[j] {
                // visible_trees += 1;
                is_visible_grid[i][j] = true;
                max_heights[j] = input[i][j];
            }
        }
    }
    // Check visibility from the left
    let mut max_heights = vec![-1; rows];
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] > max_heights[i] {
                is_visible_grid[i][j] = true;
                max_heights[i] = input[i][j];
            }
        }
    }
    // Check visibility from the right
    let mut max_heights = vec![-1; rows];
    for i in 0..rows {
        for j in (0..cols).rev() {
            if input[i][j] > max_heights[i] {
                is_visible_grid[i][j] = true;
                max_heights[i] = input[i][j];
            }
        }
    }
    let mut high_score = 0;
    for i in 0..rows {
        for j in 0..cols {
            //check score from left:
            let mut left_score = 0;
            for k in (0..j).rev() {
                if input[i][k] < input[i][j] {
                    left_score += 1;
                } else {
                    left_score += 1;
                    break;
                }
            }
            //check score from right:
            let mut right_score = 0;
            for k in (j + 1)..cols {
                if input[i][k] < input[i][j] {
                    right_score += 1;
                } else {
                    right_score += 1;
                    break;
                }
            }
            //check score from up:
            let mut up_score = 0;
            for k in (0..i).rev() {
                if input[k][j] < input[i][j] {
                    up_score += 1;
                } else {
                    up_score += 1;
                    break;
                }
            }
            //check score from bottom:
            let mut bottom_score = 0;
            for k in (i + 1)..rows {
                if input[k][j] < input[i][j] {
                    bottom_score += 1;
                } else {
                    bottom_score += 1;
                    break;
                }
            }
            let current_score = left_score * right_score * up_score * bottom_score;
            println!(
                "{}*{}*{}*{}={}",
                left_score, right_score, up_score, bottom_score, current_score
            );
            if current_score > high_score {
                high_score = current_score;
            }
        }
    }
    println!("High Score: {}", high_score);
    let visible_count: Vec<usize> = is_visible_grid
        .iter()
        .map(|row| row.iter().filter(|&&col| col == true).count())
        .collect::<Vec<usize>>();
    println!("{:?}", visible_count);
    let visible_count: usize = visible_count.into_iter().sum();
    println!("{:?}", visible_count);
    Ok(visible_count as i32)
}

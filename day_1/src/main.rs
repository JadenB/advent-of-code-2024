use std::collections::HashMap;
use std::error::Error;
use std::vec::Vec;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct ProcessedInput {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl ProcessedInput {
    fn from_buf(reader: impl BufRead) -> Result<Self, Box<dyn Error>> {
        let mut left = Vec::<i32>::new();
        let mut right = Vec::<i32>::new();

        for line in reader.lines() {
            let line = line?;
            let mut line_split = line.split_whitespace();
            left.push(
                line_split
                    .next()
                    .ok_or("missing left number")?
                    .parse::<i32>()?,
            );
            right.push(
                line_split
                    .next()
                    .ok_or("missing right number")?
                    .parse::<i32>()?,
            );
        }

        left.sort();
        right.sort();

        Ok(Self { left, right })
    }
}

fn compute_difference(input: &ProcessedInput) -> i32 {
    input
        .left
        .iter()
        .zip(input.right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn compute_similarity_score(input: &ProcessedInput) -> i32 {
    let mut right_occurances = HashMap::new();
    for val in input.right.iter() {
        right_occurances
            .entry(val)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    input
        .left
        .iter()
        .map(|e| e * right_occurances.get(&e).unwrap_or(&0))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = ProcessedInput::from_buf(BufReader::new(File::open("input/input.txt")?))?;

    // Part 1
    let difference = compute_difference(&input);
    println!("{difference}");

    // Part 2
    let similarity_score = compute_similarity_score(&input);
    println!("{similarity_score}");

    Ok(())
}

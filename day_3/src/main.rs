use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn parse_all_muls(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [left, right]) = c.extract();
            match (left.parse::<i64>(), right.parse::<i64>()) {
                (Ok(l), Ok(r)) => l * r,
                (_, _) => 0,
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    BufReader::new(File::open("input/input.txt")?).read_to_string(&mut input)?;

    // Part 1
    let result = parse_all_muls(&input);
    println!("{result}");

    // Part 2
    // Between "do()"s, the first slice before a "don't()" will be enabled
    let result: i64 = input
        .split("do()")
        .map(|s| s.split("don't()").next().map_or(0, parse_all_muls))
        .sum();
    println!("{result}");

    Ok(())
}

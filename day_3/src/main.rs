use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn parse_all_muls(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result: i64 = 0;
    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        if let (Ok(l), Ok(r)) = (left.parse::<i64>(), right.parse::<i64>()) {
            result += l * r;
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    BufReader::new(File::open("input/input.txt")?).read_to_string(&mut input)?;

    // Part 1
    let result = parse_all_muls(&input);
    println!("{result}");

    // Part 2
    let mut result: i64 = 0;
    for do_split in input.split("do()") {
        // Between "do()"s, the first slice before a "don't()" will be enabled
        let enabled_part = do_split.split("don't()").next();
        result += enabled_part.map_or(0, parse_all_muls)
    }
    println!("{result}");

    Ok(())
}
